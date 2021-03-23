// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use core::Keccak;

pub struct MerklePatriciaTree;

/// Merkle inclusivity proof
pub struct MarkleProof(Keccak, Vec<Keccak>);