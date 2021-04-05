// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use std::error::Error;

use networking::{Config, NetworkInterface};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let _network = NetworkInterface::new(Config::default());
  Ok(())
}

#[cfg(test)]
mod tests {
  use std::{
    sync::Arc,
    time::{Duration, Instant},
  };

  use auction::TransactionsAuction;
  use ethereum::{Transaction, U256};
  use oe4_runtime::{receive, send};

  #[tokio::test(flavor = "multi_thread")]
  async fn transaction_auction_io() {
    let auction = Arc::new(TransactionsAuction::new());

    let ref1 = auction.clone();
    let task1 = tokio::task::spawn(async move {
      std::thread::sleep(Duration::from_secs(1));
      let t1 = Transaction {
        nonce: U256::from(1),
        ..Transaction::default()
      };

      std::thread::sleep(Duration::from_secs(1));
      let t2 = Transaction {
        nonce: U256::from(2),
        ..Transaction::default()
      };

      // send a transaction to the auction/pool
      send(&*ref1, t1).await;
      send(&*ref1, t2).await;
    });

    let ref2 = auction.clone();
    let task2 = tokio::task::spawn(async move {
      std::thread::sleep(Duration::from_secs(1));
      let t3 = Transaction {
        nonce: U256::from(3),
        ..Transaction::default()
      };

      std::thread::sleep(Duration::from_secs(2));
      let t4 = Transaction {
        nonce: U256::from(4),
        ..Transaction::default()
      };

      // send a transaction to the auction/pool
      send(&*ref2, t3).await;
      send(&*ref2, t4).await;
    });

    let start = Instant::now();
    let proposal = receive(&*auction).await.unwrap();
    assert_eq!(proposal.len(), 3);
    assert!(start.elapsed() >= Duration::from_secs(2));

    assert_eq!(proposal[0].nonce, U256::from(1));
    assert_eq!(proposal[1].nonce, U256::from(2));
    assert_eq!(proposal[2].nonce, U256::from(3));

    task1.await.unwrap();
    task2.await.unwrap();
  }
}
