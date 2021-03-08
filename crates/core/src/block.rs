// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use crate::Transaction;
use crate::{Address, Bloom, Keccak, U256};

use serde::{Deserialize, Serialize};

/// https://ethereum.stackexchange.com/questions/268/ethereum-block-architecture
#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockHeader {
  pub number: u64,
  pub timestamp: u64,
  pub author: Address,
  pub parent_hash: Keccak,
  pub ommers_hash: Keccak,

  pub state_root: Keccak,
  pub receipts_root: Keccak,
  pub transactions_root: Keccak,
  pub logs_bloom: Bloom,

  pub gas_used: U256,
  pub gas_limit: U256,
  pub difficulty: U256,
  pub mix_hash: Keccak,
  pub nonce: u64,

  pub extra_data: [u8; 32],
}

/// https://ethereum.stackexchange.com/questions/268/ethereum-block-architecture
#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
  pub header: BlockHeader,
  pub ommers: Vec<Block>,
  pub transactions: Vec<Transaction>,
}
