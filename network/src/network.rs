// based on example of libp2p doc: https://github.com/libp2p/rust-libp2p/tree/master/examples/chat

use futures::stream::StreamExt;
use libp2p::{
    gossipsub, mdns, noise, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux, Swarm,
};
use once_cell::sync::Lazy;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::{io, select};

// Topic static variables
pub static ACCOUNT_CREATION_TOPIC: Lazy<gossipsub::IdentTopic> =
    Lazy::new(|| gossipsub::IdentTopic::new("account_creation"));
pub static TRANSFER_TOPIC: Lazy<gossipsub::IdentTopic> =
    Lazy::new(|| gossipsub::IdentTopic::new("transfer"));
pub static BALANCE_QUERY_TOPIC: Lazy<gossipsub::IdentTopic> =
    Lazy::new(|| gossipsub::IdentTopic::new("balance_query"));

/// Custom network behaviour
#[derive(NetworkBehaviour)]
struct MyBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

/// Represent a network node
///
/// It contains the libp2p swarm that will handle the network communication.
/// It should be able to send messages on topics and listen for incoming messages.
pub struct NetworkNode {
    pub swarm: Swarm<MyBehaviour>,
}

impl NetworkNode {
    /// Instanctiate a new network node
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let mut swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_quic()
            .with_behaviour(|key| {
                // To content-address message, we can take the hash of message and use it as an ID.
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };

                // Set a custom gossipsub configuration
                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                    .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
                    .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
                    .build()
                    .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?; // Temporary hack because `build` does not return a proper `std::error::Error`.

                // build a gossipsub network behaviour
                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )?;

                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?;
                Ok(MyBehaviour { gossipsub, mdns })
            })?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        // Create a Gossipsub topic
        let topics = [
            ACCOUNT_CREATION_TOPIC.clone(),
            TRANSFER_TOPIC.clone(),
            BALANCE_QUERY_TOPIC.clone(),
        ];

        for topic in topics {
            swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
        }

        Ok(Self { swarm })
    }

    /// Setup the node to listen for incoming messages
    pub async fn listen(&mut self) -> Result<(), Box<dyn Error>> {
        // Listen on all interfaces and whatever port the OS assigns
        self.swarm
            .listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        // Kick it off
        loop {
            select! {
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discovered a new peer: {peer_id}");
                            self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discover peer has expired: {peer_id}");
                            self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Local node is listening on {address}");
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => println!(
                            "Got message: {} with id: {id} from peer: {peer_id}",
                            String::from_utf8_lossy(&message.data),
                        ),
                    _ => {}
                }
            }
        }
    }

    /// Send a balance query message to the network
    pub fn send_balance_query(&mut self) {
        // Publish the message on the account balances topic
        if let Err(e) = self
            .swarm
            .behaviour_mut()
            .gossipsub
            .publish(BALANCE_QUERY_TOPIC.clone(), "coucou".as_bytes())
        {
            println!("Publish error: {e}")
        };
    }
}
