// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

use async_trait::async_trait;
use ethereum::Transaction;
use oe4_runtime::buffers;

pub type BlockProposal = Vec<Transaction>;

/// This type is responsible for selecting the most appropriate set of transactions
/// to be included in the next block.
///
/// In this first iteration it has a basic mock logic, that just porposes a
/// new block for every set of 3 transactions.
pub struct TransactionsAuction {
  incoming_tx: buffers::UnboundedBuffer<Transaction>,
}

impl TransactionsAuction {
  pub fn new() -> Self {
    TransactionsAuction {
      incoming_tx: buffers::UnboundedBuffer::new(),
    }
  }

  /// adds a transaction to the auction as a candidate for the next
  /// block that will be proposed
  pub async fn include_transaction(&self, tx: Transaction) {
    buffers::send(&self.incoming_tx, tx).await;
  }
}

#[async_trait]
impl buffers::Target<Transaction> for TransactionsAuction {
  async fn accept(&self, message: oe4_runtime::Message<Transaction>) -> oe4_runtime::MessageStatus {
    self.incoming_tx.accept(message).await
  }
}

#[async_trait]
impl buffers::Source<BlockProposal> for TransactionsAuction {
  fn try_consume(&self) -> Option<oe4_runtime::Message<BlockProposal>> {
    None
  }

  /// Aggregate transactions until 3 txs are available and then return them as one proposal
  async fn consume(&self) -> buffers::Result<oe4_runtime::Message<BlockProposal>> {
    let mut txs = Vec::with_capacity(3);
    while txs.len() != 3 {
      txs.push(self.incoming_tx.consume().await?.release());
    }
    txs.sort_by(|a, b| a.nonce.cmp(&b.nonce));
    Ok(buffers::Message::new(txs))
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
