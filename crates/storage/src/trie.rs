// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use ethereum::Keccak;

#[allow(dead_code)]
pub struct MerklePatriciaTree;

/// Merkle inclusivity proof
#[allow(dead_code)]
pub struct MarkleProof(Keccak, Vec<Keccak>);
