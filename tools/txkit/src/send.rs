// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use crate::CommonOptions;
use clap::Clap;
use ethereum::{Address, H256, U256};
use std::{
  error::Error,
  net::{SocketAddr, ToSocketAddrs},
};

#[derive(Clap, Debug)]
pub(crate) struct SendOptions {
  #[clap(long, default_value = "1")]
  nonce: U256,

  #[clap(long, default_value = "5208")]
  gas_limit: U256, // default transfer price 21000 wei

  #[clap(long, default_value = "3B9ACA00")]
  gas_price: U256, // default is 1 Gwei

  #[clap(long, default_value = "DE0B6B3A7640000")]
  value: U256, // default is 1 Ether

  #[clap(long)]
  to: Address,

  #[clap(long, default_value = "1")]
  chain: u64, // default is 1 mainnet

  #[clap(
    long,
    parse(try_from_str = parse_target_addr),
    about="address of the ethereum client node",
    default_value = "localhost:8545")]
  target: SocketAddr,

  #[clap(
    long,
    about = "Secret 256-bit key used as x in an ECDSA signature",
    default_value = "0000000000000000000000000000000000000000000000000000000000000001"
  )]
  secret: H256, // this default gives sender addr: 0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf
}

pub(crate) async fn run(opts: &SendOptions, _: &CommonOptions) -> Result<(), Box<dyn Error>> {
  let tx = ethereum::Transaction::new(
    opts.nonce,
    opts.gas_price,
    opts.gas_limit,
    opts.to,
    opts.value,
    Vec::new(),
    opts.chain,
    opts.secret,
  )?;
  println!("about to send transaction: {:#?}", &tx);
  println!(
    "serialized form: {}",
    hex::encode(ethereum::rlp_serialize(&tx)?)
  );
  Ok(())
}

fn parse_target_addr(s: &str) -> Result<SocketAddr, std::io::Error> {
  Ok(
    s.to_socket_addrs()?
      .next()
      .ok_or(std::io::ErrorKind::InvalidData)?,
  )
}
