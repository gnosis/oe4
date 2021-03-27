# OpenEthereum Agents runtime

This crate implements the basic of the data-flow based async agents runtime for the upcoming OpenEtherem architecture.

The basic idea behind this architecture is to split the system into separate autonomous agents that talk to each other through messages passing. Individual agents can run all locally on one machine or be distributed over a cluster of machines forming one clustered client.

Parts of this design are inspired by 
  - [ConcRT C++ Asynchronous Agents Library](https://docs.microsoft.com/en-us/cpp/parallel/concrt/asynchronous-agents-library)
  - [Intel Thread Building Blocks Data Flow Graph](https://software.intel.com/content/www/us/en/develop/documentation/tbb-documentation/top/intel-threading-building-blocks-developer-guide/parallelizing-data-flow-and-dependence-graphs/parallelizing-data-flow-and-dependency-graphs.html)


Interaction with individual agents happens by sending them messages through `send()` or awaiting messages from them through `receive()`. Agents may publish and consume messages by implementing traits `Target` and `Source`. A high-level example of how this model works in practice is illustrated below using an example of a networking agent and a transaction pool then miner:

```rust
  let network = NetworkInterface::new();
  let auction = TransactionAuction::new();
  let miner = BlockMiner::new();

  let transaction1 = receive(&network).await?;
  let transaction2 = receive(&network).await?;
  let transaction3 = receive(&network).await?;
  
  send(&auction, transaction1).await;
  send(&auction, transaction2).await;
  send(&auction, transaction3).await;

  let block_proposal: BlockProposal = receive(&auction).await?;
  send(&miner, block_proposal).await;

  let minted_block = receive(&miner).await?;
```

You can see an actual compiling and running version of a `TransactionAuction` in the `auction` crate and an example of how its integrated with real transaction objects in the `oe` crate tests.

This kind of apporach gives us few benefits:
  - Each individual agent has its own threading model and may internally spawn several instances of itself according to its processing need (i.e. 3x EVM for each 1x Transaction Pool).
  - The overall understanding of the system is greatly simplified, because the developer now needs only to focus on the actual data flow of transformations. For example define that 
    - a transaction comes from the network then goes to pool -> evm -> miner -> rpc -> stop.
    - a block comes from the network then goes to integrity verifier -> evm -> merkle proof verifier -> storage agent.
    - model things accordingly as a combination of `send`/`receive` between individual agents.
  - It is transparent to individual agents whether the messages are coming from shared memory, network, IPC, or through other means, this opens the door to have different pluggable implementations of individual components without even shutting down the system. For example plugging a network-local redis-based key-value store, or a message-based pool of local miners, or an rpc interface behind nginx, etc.


## Working Code Example:

```rust
#[cfg(test)]
mod tests {
  use std::{
    sync::Arc,
    time::{Duration, Instant},
  };

  use auction::TransactionsAuction;
  use ethereum::{Transaction, U256};
  use oe4_runtime::{receive, send};

  #[tokio::test(flavor = "multi_thread")]
  async fn transaction_auction_io() {
    let auction = Arc::new(TransactionsAuction::new());

    let ref1 = auction.clone();
    let task1 = tokio::task::spawn(async move {
      std::thread::sleep(Duration::from_secs(1));
      let t1 = Transaction {
        nonce: U256::from(1),
        ..Transaction::default()
      };

      std::thread::sleep(Duration::from_secs(1));
      let t2 = Transaction {
        nonce: U256::from(2),
        ..Transaction::default()
      };

      // send a transaction to the auction/pool
      send(&*ref1, t1).await;
      send(&*ref1, t2).await;
    });

    let ref2 = auction.clone();
    let task2 = tokio::task::spawn(async move {
      std::thread::sleep(Duration::from_secs(1));
      let t3 = Transaction {
        nonce: U256::from(3),
        ..Transaction::default()
      };

      std::thread::sleep(Duration::from_secs(2));
      let t4 = Transaction {
        nonce: U256::from(4),
        ..Transaction::default()
      };

      // send a transaction to the auction/pool
      send(&*ref2, t3).await;
      send(&*ref2, t4).await;
    });

    let start = Instant::now();
    let proposal = receive(&*auction).await.unwrap();
    assert_eq!(proposal.len(), 3);
    assert!(start.elapsed() >= Duration::from_secs(2));

    assert_eq!(proposal[0].nonce, U256::from(1));
    assert_eq!(proposal[1].nonce, U256::from(2));
    assert_eq!(proposal[2].nonce, U256::from(3));

    task1.await.unwrap();
    task2.await.unwrap();
  }
}
```

The implementation of a transaction pool/auction in this model looks like this:

```rust
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

```


See also:

- [Async Message Buffers Library](src/buffers/mod.rs)
- [UnboundedBuffer](src/buffers/unbounded.rs)
- [OverwriteBuffer](src/buffers/overwrite.rs)
- [WriteOnceBuffer](src/buffers/write_once.rs)
- [Message](src/buffers/message.rs)