// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use super::{Message, Source, Target};

use crossbeam_queue::SegQueue;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::{Arc, RwLock};

/// An unbounded buffer of messages of type T.
/// Messages are forwarded in arbitrary order.
pub struct UnboundedBuffer<T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned,
{
  /// messages that were not consumed yet by targets
  pending: SegQueue<Message<T>>,

  /// all targets that consume messages from this buffer
  targets: RwLock<Vec<Arc<dyn Target<T>>>>,
}

impl<T> Source<T> for UnboundedBuffer<T> where T: Sized + Send + Clone + Serialize + DeserializeOwned
{}

impl<T> Target<T> for UnboundedBuffer<T> where T: Sized + Send + Clone + Serialize + DeserializeOwned
{}
