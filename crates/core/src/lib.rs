// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

mod account;
mod block;
mod primitive;
mod rlp;
mod transaction;

// large integers
pub use primitive::{Address, Bloom, H256, H256 as Keccak, U256};

// domain types
pub use account::Account;
pub use block::Block;
pub use transaction::Transaction;

// rlp de/serialization
// TODO: consider isolating into a separate crate
pub use crate::rlp::{
  deserialize as rlp_deserialize, deserialize_from as rlp_deserialize_from,
  serialize as rlp_serialize,
};