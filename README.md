This is a basic implementation of a cryptocurrency in Rust. Here's a breakdown of the main components:

Transaction: Represents a transfer of coins from a sender to a recipient.
Block: Contains a list of transactions and metadata like index, timestamp, and previous block's hash.
Blockchain: Manages the chain of blocks and current pending transactions.

The main features implemented are:

1) Creating new transactions
2) Mining new blocks (with a simple Proof of Work algorithm)
3) Calculating block hashes
4) Maintaining the blockchain

Add these dependencies to your `Cargo.toml:`

`[dependencies]`
sha2 = "0.10.6"
chrono = "0.4.23"

And, then Run the project with `cargo run`

This implementation is simplified and not secure enough for real-world use, but it demonstrates the core concepts of a blockchain-based cryptocurrency.
