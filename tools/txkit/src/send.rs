// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use crate::CommonOptions;
use clap::Clap;
use ethereum::{Address, U256};
use secp256k1::key::SecretKey;
use std::{error::Error, io::ErrorKind, net::{SocketAddr, ToSocketAddrs}};

#[derive(Clap, Debug)]
pub(crate) struct SendOptions {
  #[clap(long, default_value="1")]
  nonce: U256,

  #[clap(long, default_value="5208")]
  gas_limit: U256, // default transfer price 21000 wei

  #[clap(long, default_value="3B9ACA00")]
  gas_price: U256, // default is 1 Gwei

  #[clap(long, default_value="DE0B6B3A7640000")]
  value: U256, // default is 1 Ether

  #[clap(long)]
  to: Address,

  #[clap(long, default_value="1")]
  chain: u64, // default is 1 mainnet

  #[clap(
    long,
    parse(try_from_str = parse_target_addr),
    about="address of the ethereum client node",
    default_value = "localhost:8545")]
  target: SocketAddr,

  #[clap(
    long, 
    parse(try_from_str = parse_secret_key),
    about="Secret 256-bit key used as x in an ECDSA signature",
    default_value="0000000000000000000000000000000000000000000000000000000000000001")]
  secret: SecretKey // this default gives sender addr: 0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf
}

pub(crate) async fn run(opts: &SendOptions, global: &CommonOptions) -> Result<(), Box<dyn Error>> {
  println!("global.opts: {:?}", &global);
  println!("send.opts: {:#?}", &opts);
  Ok(())
}

fn parse_target_addr(s: &str) -> Result<SocketAddr, std::io::Error> {
  Ok(
    s.to_socket_addrs()?
      .next()
      .ok_or(std::io::ErrorKind::InvalidData)?,
  )
}

fn parse_secret_key(s: &str) -> Result<SecretKey, std::io::Error> {
  let mut key_bytes = [0u8; secp256k1::constants::SECRET_KEY_SIZE];
  let mut bytes = hex::decode(s).map_err(|e| std::io::Error::new(ErrorKind::InvalidInput, e))?;
  bytes.resize(key_bytes.len(), 0);
  key_bytes.copy_from_slice(&bytes);
  Ok(key_bytes.into())
}