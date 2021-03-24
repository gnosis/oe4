// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use std::marker::PhantomData;

/// Performs work when it receives data and sends the
/// result of that work to another buffer.
///
/// The [TransformBuffer] class can act on different
/// input and output types.
pub struct TransformBuffer<In, Out> {
  _unused1: PhantomData<In>,
  _unused2: PhantomData<Out>,
}
