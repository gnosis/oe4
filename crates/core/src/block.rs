// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use super::{Address, H256};

pub trait Block {
  fn parent_hash(&self) -> H256;
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
