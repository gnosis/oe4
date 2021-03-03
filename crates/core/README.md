# OpenEthereum 4.0 Core

This crate contains the most fundamental open ethereum types and should have no dependencies on any other crates in this project.

Its purpose is to define a unified type model for common data types used across various components of the system.

It is expected that almost all other crates will have this one as a dependency.


## Design notes:

  - This crate implements large integer types. It is inspired by Parity's ethereum-types, however its trimmed down version is used for use with `serde-rlp` serialization and it also fixes compilation issues for rust 2018 edition.

  - All serialization to and from JSON, RLP (or the proposed Blob Serialization) should be handled at the Serde level and new formats should be just serde drivers.

  - Currently exported types are 
    - large ints: `U128`, `U256`, `U512` 
    - hashes: `H128`, `H160`, `H256`, `H264`, `H512`
    - fundamental concepts: `Block`, `Transaction`, _any additional functionalities to those objects should be added through traits. Please do your best not to duplicate those types. If you feel a strong urge to create a new fundamental concept type please ask around the team first._
