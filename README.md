# Blockchain test

This repository contains a simple implementation of a blockchain in Rust.

## Specifications

In this assignment, we will build a toy blockchain called ‘B’.

Like other blockchains, B creates new blocks. Therefore, when we send a transaction command, it takes a few seconds to be confirmed because the transaction needs to be included in a new block. As on some real blockchains, B creates new blocks at regular time intervals of 10 seconds. So, let’s say blocks are minted at T=10, T=20, T=30, etc. If we send a transaction a T=7, we will wait 3 seconds for its confirmation. If we send one at T=12, we will wait 8 seconds for the transaction to be confirmed in a new block.

There are two types of transactions on B, one for creating accounts and the other for transferring funds.

There is also a read command for viewing an account balance. However, it is a read command, not a transaction. So the balance command should instantaneously show the result.

Here are its desired features:

1. `b start-node`: The `start-node` command starts a local, new B blockchain server. Keep it running in a separate terminal. It should stop with Ctrl-C.
2. `b create-account <id-of-account> <starting-balance>`: The `create-account` transaction should create an account on B.
3. `b transfer <from-account> <to-account> <amount>`: The `transfer` transaction should send funds from one account to another on B.
4. `b balance <account>`: The `balance` command should display the funds of a B account. Remember, this is a read command.

**Miscellaneous**:

Display meaningful error messages only if the user misuses a command. You do not have to handle other errors.

The B simulation is a local, single-threaded CLI. There is no need for cryptography! Account information is not permanently stored, as the `start-node` command will start a new blockchain.

As long as the four commands work as expected, there is no single “right” way of doing this simulation project 🙂

## TODO

### V1

Assertions: this won't handle multiple nodes (no synchronization), and won't use cryptography.

- Blockchain

  - Setup transactions
  - Setup blocks
  - Setup blockchain

- Node

  - Transactions pool
  - Blocks mining

- Network

  - Setup P2P network
  - Setup topic for account creation
  - Setup topic for transfer
  - Setup topic for balance

- Cli
  - Handle `start-node` command
  - Handle `create-account` command
  - Handle `transfer` command
  - Handle `balance` command

### V2

Maybe add some synchronization between nodes, and use cryptography ?
