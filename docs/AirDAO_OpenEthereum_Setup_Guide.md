# AirDAO (ex Ambrosus) OpenEthereum Setup Guide

This guide provides step-by-step instructions for setting up Ambrosus OpenEthereum on your server.

## Prerequisites

Before you begin, ensure that you have the following prerequisites:

- Access to a Linux server
- Docker installed on your server
- Basic knowledge of using the command line

## Installation Steps

Follow these steps to set up AirDAO (ex Ambrosus) OpenEthereum:

1. Create necessary directories:
   ```shell
   mkdir server server/data
   cd server/data/


2. Download the blockchain data backup::

    ```shell
    curl -s https://backup.ambrosus.io/blockchain.tgz | sudo tar zxpf -
    ```

4. Fetch the AirDAO (ex Ambrosus) chain specification `chain.json`:

    ```shell
    curl -o chain.json https://chainspec.ambrosus.io/
    ```

5. Create Parity configuration file `parity_config.toml`:

    ```shell
    nano parity_config.toml
    ```

    and put this configuration in it

     ```shell
      [parity]
      base_path = "/app/"
      mode = "active"
      chain = "/app/chain.json"
      
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

6. List the downloaded files:

    ```shell
    ls
    ```

    You should see the: `chain.json`, `chains`, `parity_config.toml`.

7. Adjust permissions for the data directory `server/data`:

    ```shell
    chmod 777 -R .
    ```

8. Verify your current directory:

    ```shell
    pwd
    ```

     `/root/server/data`.

9. Start AirDAO (ex Ambrosus) OpenEthereum as a Docker container:

    ```shell
    docker run --name "amb-node" -d -v /root/server/data/:/app -p 8545:8545 ghcr.io/ambrosus/openethereum:v3.3.3-amb1.2.2.5 --config /app/parity_config.toml
    ```

10. Check if the Docker container is running:

    ```shell
    docker ps
    ```

    You should see the `amb-node` container listed with status "Up."

