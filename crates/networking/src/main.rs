// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use networking::{Config, NetworkInterface};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let network = NetworkInterface::new(Config::default()).await?;
  println!("OpenEthereum Networking Agent");
  network.run(10).await.unwrap();
  println!("done");

  Ok(())
}
