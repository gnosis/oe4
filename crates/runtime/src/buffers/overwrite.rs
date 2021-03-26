// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use super::{Message, MessageStatus, Result, Source, Target};

use async_std::sync::{Condvar, Mutex, RwLock, RwLockUpgradableReadGuard};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

/// Stores one message that can be written to and read from multiple times.
pub struct OverwriteBuffer<T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned,
{
  /// Holds the value that gets written once
  value: RwLock<Option<Message<T>>>,

  /// used to notify anyone attempting to receive before the value is set
  notify: (Mutex<()>, Condvar),
}

impl<T> OverwriteBuffer<T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned,
{
  pub fn new() -> Self {
    OverwriteBuffer {
      value: RwLock::new(None),
      notify: (Mutex::new(()), Condvar::new()),
    }
  }
}

#[async_trait]
impl<T> Source<T> for OverwriteBuffer<T>
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
impl<T> Target<T> for OverwriteBuffer<T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned + Sync,
{
  /// overwrites the available message if the new one has a different
  /// id and alwyas signals changes by notifying any awaiting receives
  async fn accept(&self, message: Message<T>) -> MessageStatus {
    let access = self.value.upgradable_read().await;
    if let Some(ref existing) = *access {
      if existing.id() == message.id() {
        return MessageStatus::Declined;
      }
    }
    let mut writer = RwLockUpgradableReadGuard::upgrade(access).await;
    (*writer).replace(message);
    self.notify.1.notify_all();
    MessageStatus::Accepted
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
    let wobuf: Arc<OverwriteBuffer<u64>> = Arc::new(OverwriteBuffer::new());

    assert_eq!(wobuf.try_consume(), None);

    let wobuf_t = wobuf.clone();
    let t_write = async_std::task::spawn(async move {
      std::thread::sleep(Duration::from_secs(1));
      assert_eq!(
        wobuf_t.accept(Message::new(10)).await,
        MessageStatus::Accepted
      );
      std::thread::sleep(Duration::from_secs(1));
      assert_eq!(
        wobuf_t.accept(Message::new(20)).await,
        MessageStatus::Accepted
      );
    });

    let start = Instant::now();
    assert_eq!(*wobuf.consume().await.unwrap().payload(), 10u64);
    std::thread::sleep(Duration::from_secs(2));
    assert_eq!(*wobuf.consume().await.unwrap().payload(), 20u64); // double read
    assert_eq!(*wobuf.try_consume().unwrap().payload(), 20u64);
    assert!(start.elapsed() >= Duration::from_secs(2));
    t_write.await;
  }
}
