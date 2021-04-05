// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

mod config;

pub use config::Config;

use ethereum::{Block, Transaction};
use oe4_runtime::{async_trait, Message, Result, Source, UnboundedBuffer};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

/// Implements ethereum devp2p networking
/// It uses the sentry Rust devp2p implementation from
/// https://github.com/rust-ethereum/sentry/
///
/// This type is exposed as an async target, it allows polling
/// for new work through [receive()].
pub struct NetworkInterface {
  localnode: Arc<discv4::Node>,
  txs: UnboundedBuffer<Transaction>,
  blocks: UnboundedBuffer<Block>,
}

impl NetworkInterface {
  pub async fn new(config: Config) -> std::result::Result<Self, Box<dyn std::error::Error>> {
    Ok(NetworkInterface {
      localnode: discv4::Node::new(
        config.local_addr,
        secp256k1::SecretKey::from_slice(&config.secret_key)?,
        config.boot_nodes,
        None,
        true,
        config.local_port,
      )
      .await?,
      txs: UnboundedBuffer::new(),
      blocks: UnboundedBuffer::new(),
    })
  }

  pub async fn run(&self, iters: u64) -> std::result::Result<(), Box<dyn std::error::Error>> {
    for _ in 0..iters {
      println!("peers count: {}", self.localnode.num_nodes());
      sleep(Duration::from_secs(3)).await;
    }
    Ok(())
  }
}

#[async_trait]
impl Source<Transaction> for NetworkInterface {
  fn try_consume(&self) -> Option<Message<Transaction>> {
    self.txs.try_consume()
  }

  async fn consume(&self) -> Result<Message<Transaction>> {
    self.txs.consume().await
  }
}

#[async_trait]
impl Source<Block> for NetworkInterface {
  fn try_consume(&self) -> Option<Message<Block>> {
    self.blocks.try_consume()
  }

  async fn consume(&self) -> Result<Message<Block>> {
    self.blocks.consume().await
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
