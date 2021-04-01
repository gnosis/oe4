// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

mod error;
mod message;
mod overwrite;
mod transform;
mod unbounded;
mod write_once;

pub use message::{Message, Status as MessageStatus};
pub use overwrite::OverwriteBuffer;
pub use transform::TransformBuffer;
pub use unbounded::UnboundedBuffer;
pub use write_once::WriteOnceBuffer;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

pub type Result<T> = std::result::Result<T, error::Error>;

/// Implemented by all source buffers that propagate messages to [Target]s
#[async_trait]
pub trait Source<T> : Send + Sync
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned
{
  /// Attempts to return a message if it immediately available
  fn try_consume(&self) -> Option<Message<T>>;

  /// asynchronously blocks until a message is available
  async fn consume(&self) -> Result<Message<T>>;
}

/// Implemented by all target blocks that consume messages offered to them
/// by source buffers
#[async_trait]
pub trait Target<T> : Send + Sync
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned,
{
  /// Asynchonously sends/writes a message on the target
  async fn accept(&self, message: Message<T>) -> MessageStatus;
}

pub async fn send<T: Sized + Send + Clone + Serialize + DeserializeOwned>(
  target: &dyn Target<T>,
  value: T,
) -> MessageStatus {
  target.accept(Message::new(value)).await
}

pub fn try_receive<T: Sized + Send + Clone + Serialize + DeserializeOwned>(
  source: &dyn Source<T>,
) -> Option<T> {
  source.try_consume().map(|m| m.release())
}

pub async fn receive<T: Sized + Send + Clone + Serialize + DeserializeOwned>(
  source: &dyn Source<T>,
) -> Result<T> {
  source.consume().await.map(|m| m.release())
}


#[cfg(test)]
mod tests {

  use std::{
    sync::Arc,
    time::{Duration, Instant},
  };

  use super::*;
  use futures_await_test::async_test;

  #[async_test]
  async fn send_receive_unbounded() {
    let unbounded: Arc<UnboundedBuffer<u64>> = Arc::new(UnboundedBuffer::new());

    assert_eq!(send(&*unbounded, 10).await, MessageStatus::Accepted);
    assert_eq!(send(&*unbounded, 20).await, MessageStatus::Accepted);

    assert_eq!(receive(&*unbounded).await.unwrap(), 10);
    assert_eq!(receive(&*unbounded).await.unwrap(), 20);

    let ub_t = unbounded.clone();
    let t_insert = async_std::task::spawn(async move {
      std::thread::sleep(Duration::from_secs(3));
      assert_eq!(ub_t.accept(Message::new(30)).await, MessageStatus::Accepted);
    });

    let start = Instant::now();
    assert_eq!(receive(&*unbounded).await.unwrap(), 30);
    assert!(start.elapsed() >= Duration::from_secs(2));
    t_insert.await;
  }
}
