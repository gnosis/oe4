// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use crate::{Address, U256};
use rlp_derive::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

/// Components that constitute transaction signature
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, RlpEncodable, RlpDecodable)]
pub struct SignatureComponents {
  pub v: u8, // The V field of the signature; which half of the curve our point falls in. either 27 or 28.
  pub r: U256, // The R field of the signature; helps describe the point on the curve.
  pub s: U256, // The S field of the signature; helps describe the point on the curve.
}

/// https://ethereum.stackexchange.com/questions/268/ethereum-block-architecture
#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize, RlpEncodable, RlpDecodable)]
pub struct Transaction {
  pub nonce: U256,
  pub gas_price: U256,
  pub gas_limit: U256,
  pub to: Address,
  pub value: U256,
  pub data: Vec<u8>,
  pub signature: SignatureComponents,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rlp_serialization() {
    let tx = Transaction {
      nonce: U256::zero(),
      gas_price: U256::zero(),
      gas_limit: U256::zero(),
      to: Address::zero(),
      value: U256::zero(),
      signature: SignatureComponents {
        v: 27,
        r: U256::zero(),
        s: U256::zero(),
      },
      data: [0, 1, 2].into(),
    };

    let rlp_encoded = rlp::encode(&tx);
    let decoded: Transaction = rlp::decode(&rlp_encoded).unwrap();
    assert_eq!(decoded, tx);
  }
}
