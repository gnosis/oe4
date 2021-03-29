// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

pub(crate) struct RlpSer<T>(T);

impl<T> RlpSer<T> {
  pub fn wrap(v: T) -> Self {
    RlpSer(v)
  }
}

impl serde::Serialize for RlpSer<&[u8]> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut stream = ::rlp::RlpStream::new();
    stream.append(&self.0);
    serializer.serialize_bytes(stream.as_raw())
  }
}

#[macro_export]
macro_rules! wrap_ethereum_type {
  ($name: ident) => {
    pub use ethereum_types::$name;
    impl serde::Serialize for RlpSer<ethereum_types::$name> {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: serde::Serializer,
      {
        let mut stream = ::rlp::RlpStream::new();
        stream.append(&self.0);
        serializer.serialize_bytes(stream.as_raw())
      }
    }
  };
}

wrap_ethereum_type!(U256);
wrap_ethereum_type!(H256);
wrap_ethereum_type!(Address);
wrap_ethereum_type!(Bloom);
