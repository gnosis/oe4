// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use crate::{Address, H256, Keccak, U256};
use keccak_hash::keccak;
use rlp::RlpStream;
use serde::{Deserialize, Serialize, ser::SerializeSeq};

/// Components that constitute transaction signature
#[derive(Default, Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signature {
  pub v: u64, // The V field of the signature; which half of the curve our point falls in. either 27 or 28.
  pub r: U256, // The R field of the signature; helps describe the point on the curve.
  pub s: U256, // The S field of the signature; helps describe the point on the curve.
}

/// https://ethereum.stackexchange.com/questions/268/ethereum-block-architecture
#[derive(Default, Debug, Eq, Clone, PartialEq)]
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
    chain_id: u64,
    secret: H256,
  ) -> Result<Self, secp256k1::Error> {
    
    let mut enc_stream = RlpStream::new();
    enc_stream.begin_list(9);
    enc_stream.append(&nonce);
    enc_stream.append(&gas_price);
    enc_stream.append(&gas_limit);
    enc_stream.append(&recipient);
    enc_stream.append(&value);
    enc_stream.append(&data);
    enc_stream.append(&chain_id);
    enc_stream.append(&0u64);
    enc_stream.append(&0u64);

    let rlp = enc_stream.as_raw();
    let rlp_hash = keccak(&rlp).to_fixed_bytes();

    Ok(Transaction {
      nonce,
      gas_price,
      gas_limit,
      recipient,
      value,
      data,
      signature: Signature::new(&keccak(rlp_hash), &secret, chain_id)?,
    })
  }
}

impl Serialize for Transaction {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    use crate::primitive::RlpSer;
    let mut ser = serializer.serialize_seq(Some(9))?;
    ser.serialize_element(&RlpSer::wrap(self.nonce))?;
    ser.serialize_element(&RlpSer::wrap(self.gas_price))?;
    ser.serialize_element(&RlpSer::wrap(self.gas_limit))?;
    ser.serialize_element(&RlpSer::wrap(self.recipient))?;
    ser.serialize_element(&RlpSer::wrap(self.value))?;
    ser.serialize_element(&RlpSer::wrap(&self.data[..]))?;
    ser.serialize_element(&self.signature.v)?;
    ser.serialize_element(&RlpSer::wrap(self.signature.r))?;
    ser.serialize_element(&RlpSer::wrap(self.signature.s))?;
    ser.end()
  }
}

impl<'de> Deserialize<'de> for Transaction {
  fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    todo!()
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
  pub fn new(hash: &Keccak, secret: &H256, chain_id: u64) -> Result<Self, secp256k1::Error> {
    let ctx = secp256k1::Secp256k1::new();
    let message = secp256k1::Message::from(*hash.as_fixed_bytes());
    let secret = secp256k1::key::SecretKey::from(*secret.as_fixed_bytes());
    let s = ctx.sign_recoverable(&message, &secret)?;
    let (recovery_id, sigdata) = s.serialize_compact(&secp256k1::Secp256k1::new());
    Ok(Self {
      v: recovery_id.to_i32() as u64 + 35 + chain_id * 2,
      r: U256::from(&sigdata[0..32]),
      s: U256::from(&sigdata[32..64]),
    })
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
