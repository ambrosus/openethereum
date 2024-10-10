# AirDAO (ex Ambrosus) OpenEthereum Compilation and Installation Guide

This guide provides step-by-step instructions for compiling and installing AirDAO (ex Ambrosus) OpenEthereum from source on your Linux server.

## Prerequisites

Before you begin, ensure that you have the following prerequisites:

- Access to a Linux server
- Basic knowledge of using the command line

## Installation Steps

Follow these steps to compile and install AirDAO (ex Ambrosus) OpenEthereum:

1. Update your package list:

   ```shell
   apt-get update
   ```
2. Install necessary dependencies:

   ```shell
   apt-get install git curl build-essential cmake

3. Clone the OpenEthereum repository:

   ```shell
   git clone https://github.com/ambrosus/openethereum.git
   ```

4. Navigate to the OpenEthereum directory:

   ```shell
   cd openethereum/
   ```

5. Install Rust using Rustup (if not already installed):

   ```shell
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   source $HOME/.cargo/env
   ```

6. Install Rust version 1.47.0:

   ```shell
   rustup install 1.47.0

   rustup default 1.47.0
   ```

7. Build AirDAO (ex Ambrosus) OpenEthereum:

   ```shell
   cargo build --release --features final --target x86_64-unknown-linux-gnu
   ```

8. Create config dir

   ```shell
   mkdir app

   cd app/
   ```

9. Download the blockchain data backup:

   ```shell
   curl -s https://backup.ambrosus.io/blockchain.tgz | sudo tar zxpf -
   ```

10. Fetch the AirDAO (ex Ambrosus) chain specification chain.json:

    ```shell
    curl -o chain.json https://chainspec.ambrosus.io/
    ```

11. Create Parity configuration file parity_config.toml:

    ```shell
    nano parity_config.toml
    ```
    and put this configuration in it 

    "base_path" and "chain" need to use an absolute path ( use pwd while being in the /app/ directory )
    ```shell
      [parity]
      base_path = "/root/openethereum/app/"
      mode = "active"
      chain = "/root/openethereum/app/chain.json"
      
      [footprint]
      cache_size_db = 100
      tracing = "on"
      
      [network]
      warp = false
      
      [rpc]
      apis = ["web3", "eth", "net", "parity", "traces"]
      interface = "all"
      cors = ["*"]
      
      [websockets]
      apis = ["web3", "eth", "pubsub", "net"]
      interface = "all"
      origins = ["all"]
      
      [ipc]
      disable = true
    ```

11. List the downloaded files:

    ```shell
    ls
    ```

    You should see the: `chain.json`, `chains`, `parity_config.toml`.

12. Go back and Run AirDAO (ex Ambrosus) OpenEthereum using the provided configuration file:

    ```shell
    ./target/x86_64-unknown-linux-gnu/release/openethereum --config /root/openethereum/app/parity_config.toml
    ```

	 **_Very Important:_** The only stable and up-to-date version is the one used in the documentation, which is currently openethereum:v3.3.3-amb1.2.3.
