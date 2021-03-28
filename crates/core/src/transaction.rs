// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use crate::{Address, Keccak, H256, U256};
use keccak_hash::keccak;
use serde::{Deserialize, Serialize};

/// Components that constitute transaction signature
#[derive(Default, Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signature {
  pub v: u8, // The V field of the signature; which half of the curve our point falls in. either 27 or 28.
  pub r: U256, // The R field of the signature; helps describe the point on the curve.
  pub s: U256, // The S field of the signature; helps describe the point on the curve.
}

/// https://ethereum.stackexchange.com/questions/268/ethereum-block-architecture
#[derive(Default, Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
  pub nonce: U256,
  pub gas_price: U256,
  pub gas_limit: U256,
  pub recipient: Address,
  pub value: U256,
  pub data: Vec<u8>,
  pub signature: Signature,
}

impl Transaction {
  pub fn new(
    nonce: U256,
    gas_price: U256,
    gas_limit: U256,
    recipient: Address,
    value: U256,
    data: Vec<u8>,
    _chain_id: u64,
    secret: H256,
  ) -> Result<Self, secp256k1::Error> {
    let rlp = b"this is some rlp placeholder with chain_id";
    let rlp_hash = keccak(&rlp).to_fixed_bytes();

    Ok(Transaction {
      nonce,
      gas_price,
      gas_limit,
      recipient,
      value,
      data,
      signature: Signature::new(&keccak(rlp_hash), &secret)?,
    })
  }
}

impl Transaction {
  pub fn sender(&self) -> Address {
    todo!(); // recover
  }

  pub fn hash(&self) -> Keccak {
    todo!(); // keccak(rlp(tx))
  }
}

impl Signature {
  pub fn new(hash: &Keccak, secret: &H256) -> Result<Self, secp256k1::Error> {
    let ctx = secp256k1::Secp256k1::new();
    let message = secp256k1::Message::from(*hash.as_fixed_bytes());
    let secret = secp256k1::key::SecretKey::from(*secret.as_fixed_bytes());
    let signature = ctx.sign_recoverable(&message, &secret)?;
    Ok(signature.into())
  }
}

impl From<secp256k1::RecoverableSignature> for Signature {
  fn from(s: secp256k1::RecoverableSignature) -> Self {
    let (recovery_id, sigdata) = s.serialize_compact(&secp256k1::Secp256k1::new());

    let mut data = [0; 65];
    data[0..64].copy_from_slice(&sigdata[0..64]);
    data[64] = recovery_id.to_i32() as u8;

    Self {
      v: data[64],
      r: U256::from(&data[0..32]),
      s: U256::from(&data[32..64]),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rlp_serialization() {
    let _tx = Transaction {
      nonce: U256::zero(),
      gas_price: U256::zero(),
      gas_limit: U256::zero(),
      recipient: Address::zero(),
      value: U256::zero(),
      signature: Signature {
        v: 27,
        r: U256::zero(),
        s: U256::zero(),
      },
      data: [0, 1, 2].into(),
    };

    //let rlp_encoded = rlp::encode(&tx);
    //let decoded: Transaction = rlp::decode(&rlp_encoded).unwrap();
    //assert_eq!(decoded, tx);
  }
}
