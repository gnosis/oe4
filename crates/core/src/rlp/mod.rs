// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

pub mod des;
pub mod err;
pub mod ser;

use err::ErrorKind;

use des::EthereumRlpDeserializer;
use ser::EthereumRlpSerializer;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Result of de/serialization operations
pub type Result<T> = std::result::Result<T, err::Error>;

pub fn serialize<E>(object: &E) -> Result<Vec<u8>>
where
    E: Serialize + ?Sized,
{
    let mut serializer = EthereumRlpSerializer::new();
    object.serialize(&mut serializer)?;
    Ok(serializer.finalize())
}

pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    Ok(T::deserialize(&mut EthereumRlpDeserializer::from_slice(
        bytes,
    ))?)
}

pub fn deserialize_from<R, T>(mut reader: R) -> Result<T>
where
    R: std::io::Read,
    T: DeserializeOwned,
{
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .map_err(|e| ErrorKind::IOError(e.to_string()))?;
    deserialize(&buffer)
}

#[cfg(test)]
mod test;
