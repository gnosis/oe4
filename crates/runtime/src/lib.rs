// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

pub mod agent;
pub mod buffers;

pub use buffers::*;
pub use agent::Agent;

#[cfg(test)]
mod tests {
  use super::*;
  use serde::{Deserialize, Serialize};

  #[test]
  fn mini_agent_system_sanity() {
    
    #[derive(Clone, Serialize, Deserialize)]
    struct Transaction { 
      id: u64,
      gas: u64,
    }

    #[derive(Clone, Serialize, Deserialize)]
    struct Block {
      id: u64,
      txs: Vec<Transaction>
    }

    struct Config {
      secret_key: u64
    }

    struct NetworkInterface;
    
    struct TransactionPool {
      pub incoming_tx: buffers::UnboundedBuffer<Transaction>,
      pub proposed_block: buffers::OverwriteBuffer<Block>
    }

    

    struct EVM;
    struct Miner;
    struct Storage;


  }
}