// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use crate::Target;
use async_std::task::{self, JoinHandle};
use serde::{de::DeserializeOwned, Serialize};

pub struct ProxyNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  target: &'a dyn Target<T>,
  worker: JoinHandle<()>,
}

impl<'a, T> ProxyNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  pub fn new(target: &'a dyn Target<T>) -> Self {
    ProxyNode {
      target,
      worker: task::spawn(async {}),
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use async_std::os::unix::net::UnixStream;
  use futures_await_test::async_test;

  #[async_test]
  async fn unix_socket() -> Result<(), std::io::Error> {
    let mut stream = UnixStream::connect("/tmp/socket").await?;

    Ok(())
  }
}
