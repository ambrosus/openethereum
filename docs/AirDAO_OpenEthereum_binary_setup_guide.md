# Ascendia OpenEthereum Compilation and Installation Guide

This guide provides step-by-step instructions for compiling and installing Ascendia OpenEthereum from source on your Linux server.

## OpenEthereum Container Update Guide

[OpenEthereum Container Update Guide](#openethereum-binary-update-guide)

## Prerequisites

Before you begin, ensure that you have the following prerequisites:

- Access to a Linux server
- Basic knowledge of using the command line

## Installation Steps

Follow these steps to compile and install Ascendia OpenEthereum:

1. Update your package list:

   ```shell
   apt-get update
   ```
2. Install necessary dependencies:

   ```shell
   apt-get install git curl build-essential cmake

3. Clone the OpenEthereum repository:

   ```shell
   git clone https://github.com/ascendia-network/openethereum.git
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

7. Build Aascendia OpenEthereum:

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

10. Fetch the Aascendia chain specification chain.json:

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

12. Go back and Run Aascendia OpenEthereum using the provided configuration file:

    ```shell
    ./target/x86_64-unknown-linux-gnu/release/openethereum --config /root/openethereum/app/parity_config.toml
    ```

	 **_Very Important:_** The only stable and up-to-date version is the one used in the documentation, which is currently openethereum:v3.3.3-amb1.2.4.


# OpenEthereum Binary Update Guide

1. Stop OpenEthereum process:

   ```shell
   # First find the process PID
   ps aux | grep openethereum

   # Send a SIGTERM signal (softer termination)
   kill <PID_number>

   # Or SIGINT (equivalent to Ctrl+C)
   kill -2 <PID_number>
   ```

  If the process is running in the background or as a service:

   ```shell
   # Find the PID
   pidof openethereum
   # or
   pgrep openethereum
	
   # Then send a signal
   kill $(pidof openethereum)
   ```

2. Update the source code repository:

   ```shell
   # Switch to the root directory of the project
   cd /root/openethereum

   # Make sure you are in the main branch
   git checkout main

   # Get the latest changes
   git fetch origin

   # Switch to a specific version, such as v3.3.3-amb1.2.5
   git checkout v3.3.3-amb1.2.5 # this is now the current stable version
   ```
   

3. Recompile the project with the same parameters:

   ```shell
   # Clean the previous build
   cargo clean

   # Compile with the same parameters as before
   cargo build --release --features final --target x86_64-unknown-linux-gnu
   ```

4. Check out the new version:

   ```shell
   ./target/x86_64-unknown-linux-gnu/release/openethereum --version
   ```
   	 **_Very Important:_** After compilation, you should see a similar message where part of the commit hash (e.g. ‘e9c4f514f’) should correspond to a specific tag. You can compare hashes for each tag in our repository: https://github.com/ambrosus/openethereum/releases/. Example output:

   ```shell
   root@openethereum:~/openethereum# ./target/x86_64-unknown-linux-gnu/release/openethereum --version
   OpenEthereum Client.
     version OpenEthereum/v3.3.3-stable-e9c4f514f-20250418/x86_64-linux-gnu/rustc1.47.0
   Copyright 2015-2020 Parity Technologies (UK) Ltd.
   License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
   This is free software: you are free to change and redistribute it.
   There is NO WARRANTY, to the extent permitted by law.

   By Wood/Paronyan/Kotewicz/Drwięga/Volf/Greeff
      Habermeier/Czaban/Gotchac/Redman/Nikolsky
      Schoedon/Tang/Adolfsson/Silva/Palm/Hirsz et al.
   ```

5. Run the updated version:

   ```shell
   ./target/x86_64-unknown-linux-gnu/release/openethereum --config /root/openethereum/app/parity_config.toml
   ```

