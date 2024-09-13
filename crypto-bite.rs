//! # Simple Blockchain Implementation in Rust
//! 
//! This educational project demonstrates a basic blockchain implementation using Rust.
//! It covers core blockchain concepts such as transactions, blocks, proof of work, and
//! chain validation.
//! 
//! For detailed documentation, please refer to the `DOCUMENTATION.md` file in the project root.
//! 
//! ## Quick Start
//! 
//! To get started with this project:
//! 
//! 1. Clone the repository
//! 2. Run `cargo build` to compile the project
//! 3. Run `cargo run` to execute the example in the `main` function
//! 
//! For more information on usage and implementation details, please consult the documentation.

use sha2::{Digest, Sha256};
use chrono::Utc;

/// Represents a transaction in the blockchain
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: f64,
}

/// Represents a block in the blockchain
#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: i64,
    transactions: Vec<Transaction>,
    proof: u64,
    previous_hash: String,
}

impl Block {
    /// Creates a new block
    fn new(index: u64, transactions: Vec<Transaction>, proof: u64, previous_hash: String) -> Self {
        Block {
            index,
            timestamp: Utc::now().timestamp(),
            transactions,
            proof,
            previous_hash,
        }
    }

    /// Calculates the hash of the block
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!("{}{}{:?}{}{}", self.index, self.timestamp, self.transactions, self.proof, self.previous_hash);
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

/// Represents the blockchain
struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block
    fn new() -> Self {
        let mut chain = Vec::new();
        chain.push(Block::new(0, Vec::new(), 100, String::from("0")));
        Blockchain {
            chain,
            current_transactions: Vec::new(),
        }
    }

    /// Adds a new transaction to the list of current transactions
    fn new_transaction(&mut self, sender: String, recipient: String, amount: f64) -> usize {
        self.current_transactions.push(Transaction { sender, recipient, amount });
        self.last_block().index as usize + 1
    }

    /// Creates a new block and adds it to the chain
    fn new_block(&mut self, proof: u64) -> Block {
        let previous_hash = self.last_block().calculate_hash();
        let block = Block::new(
            self.chain.len() as u64,
            std::mem::take(&mut self.current_transactions),
            proof,
            previous_hash,
        );
        self.chain.push(block.clone());
        block
    }
    
    /// Returns a reference to the last block in the chain
    fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    /// Implements a simple proof-of-work algorithm
    fn proof_of_work(&self, last_proof: u64) -> u64 {
        let mut proof = 0;
        while !self.valid_proof(last_proof, proof) {
            proof += 1;
        }
        proof
    }

    /// Validates the proof: does hash(last_proof, proof) contain 4 leading zeroes?
    fn valid_proof(&self, last_proof: u64, proof: u64) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let guess_hash = Sha256::digest(guess.as_bytes());
        let result = format!("{:x}", guess_hash);
        result.starts_with("0000")
    }
}

fn main() {
    // Create a new blockchain
    let mut blockchain = Blockchain::new();
    
    // Mine the first block
    println!("Mining first block...");
    let last_proof = blockchain.last_block().proof;
    let proof = blockchain.proof_of_work(last_proof);
    blockchain.new_transaction(String::from("0"), String::from("Alice"), 1.0);
    let block = blockchain.new_block(proof);
    println!("New block forged: {:?}", block);

    // Mine the second block
    println!("Mining second block...");
    let last_proof = blockchain.last_block().proof;
    let proof = blockchain.proof_of_work(last_proof);
    blockchain.new_transaction(String::from("Alice"), String::from("Bob"), 0.5);
    blockchain.new_transaction(String::from("Alice"), String::from("Charlie"), 0.3);
    let block = blockchain.new_block(proof);
    println!("New block forged: {:?}", block);

    // Display the entire blockchain
    println!("Blockchain: {:?}", blockchain.chain);
}

