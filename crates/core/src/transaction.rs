// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use crate::{Address, U256};
use rlp_derive::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

/// Components that constitute transaction signature
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, RlpEncodable, RlpDecodable)]
pub struct SignatureComponents {
  pub v: u8,   // The V field of the signature; which half of the curve our point falls in.
  pub r: U256, // The R field of the signature; helps describe the point on the curve.
  pub s: U256, // The S field of the signature; helps describe the point on the curve.
}

/// https://ethereum.stackexchange.com/questions/268/ethereum-block-architecture
#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize, RlpEncodable, RlpDecodable)]
pub struct Transaction {
  pub nonce: U256,
  pub gas: U256,
  pub gas_price: U256,
  pub to: Address,
  pub value: U256,
  pub signature: SignatureComponents,
  pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rlp_serialization() {
    let tx = Transaction {
      nonce: U256::zero(),
      gas: U256::zero(),
      gas_price: U256::zero(),
      to: Address::zero(),
      value: U256::zero(),
      signature: SignatureComponents {
        v: 1,
        r: U256::zero(),
        s: U256::zero(),
      },
      data: [0, 1, 2].into(),
    };

    let rlp_encoded = rlp::encode(&tx);
    println!("bytes: {:?}", &rlp_encoded);
  }

  #[test]
  fn test_rlp_deserialization() {
    let rlp = hex::decode("f87c80018261a894095e7baea6a6c7c4c2dfeb977efac326af552d870a9d00000000000000000000000000000000000000000000000000000000001ba048b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353a0efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c804").unwrap();
    let decoded: Transaction = rlp::decode(&rlp).unwrap();

    println!("rlp bytes: {:?}", &rlp);
    println!("decoded tx: {:#?}", &decoded);
  }
}
