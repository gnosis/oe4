// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

#[derive(Debug)]
pub enum Error {
  Unknown,
  Custom(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Agent Buffer Error: {:?}", &self))
    }
}

impl std::error::Error for Error {}

impl std::convert::From<&dyn std::error::Error> for Error {
    fn from(e: &dyn std::error::Error) -> Self {
        Error::Custom(format!("{:?}", e.clone()))
    }
}

unsafe impl Send for Error {}