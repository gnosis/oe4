# OpenEthereum 4.0 Storage

This crate is responsible for implementing all storage mechanism used within OpenEthereum. There are two primary types of storage:

  - Blockchain storage, responsible for storing a copy of all known blocks and transactions.
  - State storage, responsible for storing the world state (the distributed ledger and contract state).

## Design notes
  - Flat-DB, cache merkle-tree hashes on intermediate nodes.
  - Prunning and compression,
  - Marking regions of storage that are accessed by certain contracts and applying pruning/locality/paging policies.
  - This crate should have dependency only on the `core` create and is populated by applying `StateDiff`s.
  - Storage have caches for things such as: `nonce`s, recent state, etc. 
