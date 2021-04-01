// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use crate::{MessageStatus, Target};
use async_std::sync::RwLock;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

/// A single-input, single-output node that broadcasts each message received to all successors.
/// Its input and output are of the same generic type. It does not buffer messages.
pub struct BroadcastNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned,
{
  outputs: RwLock<Vec<&'a dyn Target<T>>>,
}

impl<'a, T> BroadcastNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned,
{
  /// Initialize a new broadcast node with no predefined output targets.
  /// All incoming messages will be rejected unless at leas one output
  /// target is added.
  pub fn empty() -> Self {
    BroadcastNode {
      outputs: RwLock::new(vec![]),
    }
  }

  /// Initialize a new broadcast node with a predefined list of targets.
  /// New targets could still be appended to this node after init.
  pub fn new(outputs: Vec<&'a dyn Target<T>>) -> Self {
    BroadcastNode {
      outputs: RwLock::new(outputs),
    }
  }

  /// Adds a new target to the list of outputs post init.
  pub async fn add_target(&self, target: &'a dyn Target<T>) {
    let mut outputs_access = self.outputs.write().await;
    outputs_access.push(target);
  }
}

#[async_trait]
impl<'a, T> Target<T> for BroadcastNode<'a, T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned,
{
  async fn accept(&self, message: crate::Message<T>) -> MessageStatus {
    let outputs_access = self.outputs.read().await;
    if outputs_access.is_empty() {
      MessageStatus::Declined
    } else {
      for out in outputs_access.iter() {
        let local_copy = message.clone();
        out.accept(local_copy).await;
      }
      MessageStatus::Accepted
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::buffers::*;
  use futures_await_test::async_test;

  #[async_test]
  async fn const_init() -> Result<()> {
    let b = UnboundedBuffer::<u64>::new();
    let c = UnboundedBuffer::<u64>::new();
    let a = BroadcastNode::new(vec![&b, &c]);

    assert_eq!(send(&a, 10u64).await, MessageStatus::Accepted);
    assert_eq!(send(&a, 20u64).await, MessageStatus::Accepted);

    assert_eq!(receive(&b).await?, 10);
    assert_eq!(receive(&b).await?, 20);

    assert_eq!(receive(&c).await?, 10);
    assert_eq!(receive(&c).await?, 20);

    Ok(())
  }

  #[async_test]
  async fn dynamic_add() -> Result<()> {
    let b = UnboundedBuffer::<u64>::new();
    let c = UnboundedBuffer::<u64>::new();

    let a = BroadcastNode::empty();

    a.add_target(&b).await;
    a.add_target(&c).await;

    assert_eq!(send(&a, 10u64).await, MessageStatus::Accepted);
    assert_eq!(send(&a, 20u64).await, MessageStatus::Accepted);

    assert_eq!(receive(&b).await?, 10);
    assert_eq!(receive(&b).await?, 20);

    assert_eq!(receive(&c).await?, 10);
    assert_eq!(receive(&c).await?, 20);

    Ok(())
  }

  #[async_test]
  async fn reject_when_no_outputs() {
    let a = BroadcastNode::empty();
    assert_eq!(send(&a, 10).await, MessageStatus::Declined);
  }
}
