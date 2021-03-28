// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use crate::{Keccak, U256};
use serde::{Deserialize, Serialize};

/// https://ethereum.stackexchange.com/questions/268/ethereum-block-architecture
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Account {
  pub nonce: U256,
  pub balance: U256,
  pub storage_root: Keccak,
  pub code_hash: Keccak,
}
