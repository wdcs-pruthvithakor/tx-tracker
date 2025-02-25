# Web3 Transaction Listener

A simple Rust application that listens for Ethereum transactions sent to a specific address using WebSockets.

## Features

- Connects to an Ethereum node via WebSocket.
- Listens for new pending transactions.
- Prints transaction details when a transaction is sent to a specific Ethereum address.

## Requirements

- **Rust** (1.58.0 or later)
- **Web3-compatible Ethereum node** (e.g., geth, parity, or Infura)
- **tokio** for async execution

## Installation

1. Make sure you have **Rust** installed. If you don't, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
   
2. Clone the repository:

   ```bash
   git clone https://github.com/wdcs-pruthvithakor/tx-tracker.git
   cd tx-tracker
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

4. To run the project, you'll need to have access to a Web3-enabled Ethereum node (such as a local node or a service like Infura). Make sure you have the WebSocket URL and the target Ethereum address you want to listen to.

## Usage

Run the project with the following command:

```bash
cargo run -- --url <WEBSOCKET_URL> --address <ETHEREUM_ADDRESS>
```

### Arguments

- `--url` or `-u`: The WebSocket URL of the Ethereum node (default: `ws://127.0.0.1:8545/`).
- `--address` or `-a`: The Ethereum address to listen for incoming transactions to. This argument is **required**.

Example:

```bash
cargo run -- --url ws://127.0.0.1:8545 --address 0xa7df9c8ddbb456859f371b0105c40dc934e80211
```

This will start the listener and print details of any transactions sent to the address `0xa7df9c8ddbb456859f371b0105c40dc934e80211` as they are received by the node.

### Sample Output

```bash
Listening for transactions to 0xa7df9c8ddbb456859f371b0105c40dc934e80211
New Transaction Received!
Tx Hash: 0x3c2340db15dc8a05b9e0cf461c9894a723195e28d84d76c754881bfab1382251
From: 0xbda5747bfd65f08deb54cb465eb87d40e51b196e
To: 0xa7df9c8ddbb456859f371b0105c40dc934e80211
Value: 35.7543755584 ETH
```

In this example, the program prints out the transaction hash, sender's address, recipient's address, and the value of the transaction in ETH.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
