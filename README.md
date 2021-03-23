# OpenEthereum 4.0

OpenEthereum 4.0 - Planning &amp; Design Repository

This repository is used to create a blueprint of the system design for the new OE4 edition. At the moment it is used to gather the most important high-level design decisions.

One of the main design goals of this edition is to redesign the ethereum client into an actor-model architecture, enabling greater modularity and splitting a single client across several machines for increased capacity, which enables a cluser of machines to work together as one node. At the moment the framework of choice for implementing the actor model is [Actix](https://github.com/actix/actix), but this could change.

Please browse through individual crates for more specific discussions and/or design decisions:

  - [Core](crates/core/README.md) (fundamental types)
  - [Execution](crates/execution/README.md) (evm)
  - [Networking](crates/networking/README.md) (devp2p, libp2p, json-rpc)
  - [Storage](crates/storage/README.md) (snapshotting, import/export, state, blocks store, pruning, archival, etc.)
  - [Consensus](crates/consensus/README.md) (PoW (+ Miner), PoS, AuRa, etc..)
  - [Transaction Pool](crates/txpool/README.md)
  - [OE](crates/oe/README.md)
