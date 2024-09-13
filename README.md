This is a basic implementation of a blockchain in Rust. Here's a breakdown of the main components:

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
This is how your results show: 

This output shows the successful compilation and execution of your Rust cryptocurrency project. Let's break down what's happening:

![image](https://github.com/user-attachments/assets/d850a1a1-6086-4d6b-b456-5343bbdc34ce)

`Compilation:`
The project "Crypto_bite" was compiled successfully in debug mode, taking 2.16 seconds.

`Execution:`

The compiled executable is then run.
Blockchain Creation and Mining:

The program creates a simple blockchain and mines two blocks.
Each block shows the mining process and the resulting block details.


`Block Details:`
Each block contains:

Index: The position of the block in the chain (0, 1, 2)
Timestamp: Unix timestamp when the block was created
Transactions: List of transactions in the block
Proof: A number used in the proof-of-work system
Previous hash: Hash of the previous block, ensuring chain integrity


`Transactions:`

The first mined block (index 1) has one transaction: 1.0 coins sent from "0" (likely representing newly minted coins) to "Alice".
The second mined block (index 2) has two transactions: Alice sending 0.5 to Bob and 0.3 to Charlie.


`Final Blockchain State:`
The output shows the entire blockchain, including the genesis block (index 0) and the two mined blocks.

This output demonstrates that your cryptocurrency implementation is working as intended. It's creating blocks, processing transactions, and maintaining the blockchain structure. The proof-of-work system also functions, as evidenced by the different proof values for each block.


This implementation is simplified and not secure enough for real-world use, but it demonstrates the core concepts of a blockchain-based cryptocurrency.

