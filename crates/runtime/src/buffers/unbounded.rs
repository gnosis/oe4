// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use super::{Message, MessageStatus, Result, Source, Target};
use async_std::sync::{Condvar, Mutex};
use async_trait::async_trait;
use crossbeam_queue::SegQueue;
use serde::{de::DeserializeOwned, Serialize};

/// An unbounded buffer of messages of type T.
/// Messages are forwarded in arbitrary order.
pub struct UnboundedBuffer<T>
where
  T: Sized + Send + Serialize + DeserializeOwned,
{
  /// messages that were not consumed yet by targets
  pending: SegQueue<Message<T>>,
  /// used to signal changes to the buffer for waiting consumers
  notify: (Mutex<()>, Condvar),
}

impl<T> UnboundedBuffer<T>
where
  T: Sized + Send + Serialize + DeserializeOwned,
{
  pub fn new() -> Self {
    UnboundedBuffer {
      pending: SegQueue::new(),
      notify: (Mutex::new(()), Condvar::new()),
    }
  }

  #[cfg(test)]
  pub(crate) fn count(&self) -> usize {
    self.pending.len()
  }
}

#[async_trait]
impl<T> Source<T> for UnboundedBuffer<T>
where
  T: Sized + Send + Serialize + DeserializeOwned,
{
  fn try_consume(&self) -> Option<Message<T>> {
    self.pending.pop()
  }

  async fn consume(&self) -> Result<Message<T>> {
    if let Some(value) = self.pending.pop() {
      Ok(value)
    } else {
      let lock = self.notify.0.lock().await;
      self.notify.1.wait(lock).await;
      self.consume().await
    }
  }
}

#[async_trait]
impl<T> Target<T> for UnboundedBuffer<T>
where
  T: Sized + Send + Serialize + DeserializeOwned,
{
  async fn propagate(&self) -> MessageStatus {
    MessageStatus::Declined
  }
  async fn accept(&self, message: Message<T>) -> MessageStatus {
    self.pending.push(message);
    self.notify.1.notify_one();
    MessageStatus::Accepted
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use std::sync::Arc;

  use futures_await_test::async_test;

  #[async_test]
  async fn sanity_test() {
    let ubuf: UnboundedBuffer<u64> = UnboundedBuffer::new();
    let status = ubuf.accept(Message::new(10)).await;
    assert_eq!(status, MessageStatus::Accepted);
    assert_eq!(ubuf.count(), 1);
    let dequed = ubuf.try_consume();
    assert!(dequed.is_some());
    assert_eq!(*dequed.unwrap(), 10);
  }

  #[async_test]
  async fn mt_try_consume_test() {
    let ubuf: Arc<UnboundedBuffer<u64>> = Arc::new(UnboundedBuffer::new());

    let buf_t1 = ubuf.clone();
    let buf_t2 = ubuf.clone();
    let buf_t3 = ubuf.clone();

    let t1 = std::thread::spawn(|| async move {
      for i in 0..1000 {
        buf_t1.accept(Message::new(i)).await;
        if i % 100 == 0 {
          std::thread::yield_now();
        }
      }
    });

    let t2 = std::thread::spawn(|| async move {
      for i in 0..1000 {
        buf_t2.accept(Message::new(i)).await;
        if i % 100 == 0 {
          std::thread::yield_now();
        }
      }
    });

    t1.join().expect("t1 paniced").await;
    t2.join().expect("t2 paniced").await;

    let mut counter = 0u64;
    let mut sum = 0u64;

    for i in 0..2000u32 {
      let consumed = buf_t3.try_consume();
      assert!(consumed.is_some());
      if let Some(v) = consumed {
        counter += 1;
        sum += *v;
      }

      if i % 10 == 0 {
        std::thread::yield_now();
      }
    }

    assert_eq!(sum, 999_000);
    assert_eq!(counter, 2000);
  }

  #[async_test]
  async fn mt_consume_test() {
    let ubuf: Arc<UnboundedBuffer<u64>> = Arc::new(UnboundedBuffer::new());

    let buf_t1 = ubuf.clone();
    let buf_t2 = ubuf.clone();
    let buf_t3 = ubuf.clone();

    let t1 = std::thread::spawn(|| async move {
      for i in 0..1000 {
        buf_t1.accept(Message::new(i)).await;
        if i % 100 == 0 {
          std::thread::yield_now();
        }
      }
    });

    let t2 = std::thread::spawn(|| async move {
      for i in 0..1000 {
        buf_t2.accept(Message::new(i)).await;
        if i % 100 == 0 {
          std::thread::yield_now();
        }
      }
    });

    let t3 = std::thread::spawn(|| async move {
      let mut counter = 0u64;
      let mut sum = 0u64;

      for i in 0..2000u32 {
        let consumed = buf_t3.consume().await;
        
        counter += 1;
        sum += *consumed.unwrap();

        if i % 10 == 0 {
          std::thread::yield_now();
        }
      }

      assert_eq!(sum, 999_000);
      assert_eq!(counter, 2000);
    });

    t1.join().expect("t1 paniced").await;
    t2.join().expect("t2 paniced").await;
    t3.join().expect("t2 paniced").await;
  }
}
