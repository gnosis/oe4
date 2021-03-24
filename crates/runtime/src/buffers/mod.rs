// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

mod message;
mod overwrite;
mod transform;
mod unbounded;
mod write_once;

pub use message::Message;
pub use overwrite::OverwriteBuffer;
pub use transform::TransformBuffer;
pub use unbounded::UnboundedBuffer;
pub use write_once::WriteOnceBuffer;

/// The valid responses for an offer of a message to a block.
pub enum Status {
  /// The target accepted the message.
  Accepted,
  /// The target did not accept the message.
  Declined,
  /// The target tried to accept the message but it was no longer available.
  Missed,
  /// The target posponed the message
  Posponed,
}

/// Implemented by all source buffers that propagate messages to [Target]s
pub trait Source<T> {}

/// Implemented by all target blocks that consume messages offered to them
/// by source buffers.
pub trait Target<T> {}
