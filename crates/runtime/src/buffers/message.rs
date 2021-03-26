// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use std::ops::Deref;

use rand::RngCore;
use serde::{de::DeserializeOwned, Serialize};

/// A unique identifier of a message sent in the system
type Id = u64;

/// The basic message envelope containing the data
/// payload being passed between messaging blocks.
#[derive(Clone, Debug)]
pub struct Message<T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned,
{
  payload: T,
  id: Id,
}

impl<T> Message<T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned,
{
  pub fn new(payload: T) -> Self {
    Message {
      payload: payload,
      id: rand::thread_rng().next_u64(),
    }
  }

  pub fn payload(&self) -> &T {
    &self.payload
  }

  pub fn release(self) -> T {
    self.payload
  }

  pub fn id(&self) -> Id {
    self.id
  }
}

impl<T> PartialEq for Message<T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned,
{
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl<T> Eq for Message<T> where T: Sized + Send + Clone + Serialize + DeserializeOwned {}

impl<T> Deref for Message<T>
where
  T: Sized + Send + Clone + Serialize + DeserializeOwned,
{
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.payload
  }
}

/// The valid responses for an offer of a message to a block.
#[derive(Debug, PartialEq, Eq)]
pub enum Status {
  /// The target accepted the message.
  Accepted,
  /// The target did not accept the message.
  Declined,
  /// The target posponed the message
  Posponed,
  /// The target tried to accept the message but it was no longer available.
  Missed,
}

unsafe impl Send for Status {}
