// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use std::marker::PhantomData;

/// Stores one message that can be written to one
/// time and read from multiple times.
pub struct WriteOnceBuffer<T> {
  _unused: PhantomData<T>,
}
