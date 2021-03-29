// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use ethereum_types::U256;
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
  first_name: String,
  last_name: String,
  age: u64,
}

#[test]
fn struct_simple_test() -> super::Result<()> {
  #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
  struct Item {
    a: String,
  }

  let item = Item { a: "cat".into() };
  let expected = vec![0xc4, 0x83, b'c', b'a', b't'];
  let out = super::serialize(&item)?;

  assert_eq!(out, expected);

  let decoded = super::deserialize(&expected)?;
  assert_eq!(item, decoded);

  Ok(())
}

#[test]
fn struct_complex_test() -> super::Result<()> {
  #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
  struct Item {
    a: String,
    b: u64,
    c: ethereum_types::U256,
  }

  let item = Item {
    a: "cat".into(),
    b: 1599u64,
    c: U256::from(208090),
  };

  let out = super::serialize(&item)?;
  let deserialized: Item = super::deserialize(&out)?;
  assert_eq!(item, deserialized);

  Ok(())
}

#[test]
fn vec_u64_test() -> super::Result<()> {
  let empty_v: Vec<u64> = vec![];
  let single_v = vec![15_u64];
  let many_v = vec![1, 2, 3, 7, 0xff];

  let encoded_empty_v = super::serialize(&empty_v)?;
  let encoded_single_v = super::serialize(&single_v)?;
  let encoded_many_v = super::serialize(&many_v)?;

  assert_eq!(encoded_empty_v, hex!("c0"));
  assert_eq!(encoded_single_v, hex!("c10f"));
  assert_eq!(encoded_many_v, hex!("c60102030781ff"));

  let decoded_many_v: Vec<u64> = super::deserialize(&encoded_many_v)?;
  let decoded_empty_v: Vec<u64> = super::deserialize(&encoded_empty_v)?;
  let decoded_single_v: Vec<u64> = super::deserialize(&encoded_single_v)?;

  assert_eq!(empty_v, decoded_empty_v);
  assert_eq!(single_v, decoded_single_v);
  assert_eq!(many_v, decoded_many_v);

  Ok(())
}

#[test]
fn str_test() -> super::Result<()> {
  let s = "Lorem ipsum dolor sit amet, consectetur adipisicing elit";
  let encoded_s = super::serialize(s)?;

  assert_eq!(
    encoded_s,
    vec![
      0xb8, 0x38, b'L', b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd',
      b'o', b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm', b'e', b't', b',', b' ',
      b'c', b'o', b'n', b's', b'e', b'c', b't', b'e', b't', b'u', b'r', b' ', b'a', b'd', b'i',
      b'p', b'i', b's', b'i', b'c', b'i', b'n', b'g', b' ', b'e', b'l', b'i', b't',
    ]
  );

  let decoded_s: String = super::deserialize(&encoded_s)?;
  assert_eq!(s, decoded_s);

  Ok(())
}

#[test]
fn transaction_serialization_test() -> Result<(), Box<dyn Error>> {
  let tx = crate::Transaction::new(
    690.into(),
    2000000000.into(),
    21000.into(),
    hex!("4592d8f8d7b001e72cb26a73e4fa1806a51ac79d").into(),
    100.into(), // 1 ether
    Vec::new(),
    1,
    hex!("0000000000000000000000000000000000000000000000000000000000000001").into(),
  )?;

  let rlp = super::serialize(&tx)?;
  println!("rlp: {:?}", hex::encode(&rlp));
  assert!(false);

  Ok(())
}
