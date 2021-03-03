# OpenEthereum 4.0 Networking

This crate implements network IO. At the moment of writing the set of protocols used by Ethereum are collectively called `devp2p`, however `libp2p` is being considered as a replacement protocol suite.

## Design notes

- This crate should not have dependencies on anything other than the `core` crate.
- Most of the time this library will expose _async_ `Stream`s of things, such as:

  - ```rust 
    let network = open_network(...)?;
    while let Some(transaction) = network.transactions.next().await {
      // insert transaction into local transaction pool
      transactions_queue.append(transaction);
    }
    ```
  - ```rust 
    let network = open_network(...)?;
    while let Some(peer) = network.peers.next().await {
      // add peer to local peers or otherwise process it
      if let Ok(handshake) = syncpeers.handshake(peer)? {
        // connection established
      }
    }
    ```  
  - ```rust 
    let network = open_network(...)?;
    while let Some(block) = network.blocks.next().await {
      // import new mined block
      storage.chain.append(block)?;
    }
    ```  
- Design this crate in a way that makes it possible for use in scenarios such as:
  - Writing a tool that enumerates all known peers and return their client versions and supported protocols in as few lines of code as possible.
  - Writing a tool that imports blocks across sync protocol (warp, fastsync, etc.) for faster sync with multiple clients.
  - etc.
  - IF you think about those scenarios and how easy it would be to use this library to build those tools, that should help gauge whether the library is decoupled enough from the rest of the workspace.