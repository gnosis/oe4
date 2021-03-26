// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use super::{Message, MessageStatus, Result, Source, Target};

use async_std::sync::{Condvar, Mutex, RwLock, RwLockUpgradableReadGuard};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

/// Stores one message that can be written to one
/// time and read from multiple times.
pub struct WriteOnceBuffer<T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned,
{
  /// Holds the value that gets written once
  value: RwLock<Option<Message<T>>>,

  /// used to notify anyone attempting to receive before the value is set
  notify: (Mutex<()>, Condvar),
}

impl<T> WriteOnceBuffer<T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned,
{
  pub fn new() -> Self {
    WriteOnceBuffer {
      value: RwLock::new(None),
      notify: (Mutex::new(()), Condvar::new()),
    }
  }
}

#[async_trait]
impl<T> Source<T> for WriteOnceBuffer<T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  fn try_consume(&self) -> Option<Message<T>> {
    self.value.try_read().map_or(None, |r| r.clone())
  }

  async fn consume(&self) -> Result<Message<T>> {
    if let Some(ref message) = self.try_consume() {
      Ok(message.clone())
    } else {
      let lock = self.notify.0.lock().await;
      self.notify.1.wait(lock).await;
      self.consume().await
    }
  }
}

#[async_trait]
impl<T> Target<T> for WriteOnceBuffer<T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  /// Only accept the first assigned value, and reject all changes
  /// afterwards
  async fn accept(&self, message: Message<T>) -> MessageStatus {
    let access = self.value.upgradable_read().await;
    match *access {
      Some(_) => MessageStatus::Declined,
      None => {
        let mut writer = RwLockUpgradableReadGuard::upgrade(access).await;
        *writer = Some(message);
        self.notify.1.notify_all();
        MessageStatus::Accepted
      }
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use futures_await_test::async_test;
  use std::{
    sync::Arc,
    time::{Duration, Instant},
  };

  #[async_test]
  async fn sanity_test() {
    let wobuf: Arc<WriteOnceBuffer<u64>> = Arc::new(WriteOnceBuffer::new());

    assert_eq!(wobuf.try_consume(), None);

    let wobuf_t = wobuf.clone();
    let t_write = async_std::task::spawn(async move {
      std::thread::sleep(Duration::from_secs(3));
      assert_eq!(
        wobuf_t.accept(Message::new(10)).await,
        MessageStatus::Accepted
      );

      assert_eq!(
        wobuf_t.accept(Message::new(20)).await,
        MessageStatus::Declined
      );
    });

    let start = Instant::now();
    assert_eq!(*wobuf.consume().await.unwrap().payload(), 10u64);
    assert_eq!(*wobuf.consume().await.unwrap().payload(), 10u64); // double read
    assert_eq!(*wobuf.try_consume().unwrap().payload(), 10u64);
    assert!(start.elapsed() >= Duration::from_secs(2));
    t_write.await;
  }
}
