use libp2p::{gossipsub, mdns, swarm::NetworkBehaviour};
use network::network::NetworkNode;
use std::{error::Error, vec};

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
struct MyBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new network node and listen for incoming messages.
    let mut network_node = NetworkNode::new().await?;
    network_node.listen().await?;

    // Send a balance query to the network.
    // TODO: fix this as something is going wrong here.
    // When running two nodes, the balance query is not being received by the other node.
    network_node.send_balance_query();

    Ok(())
}
