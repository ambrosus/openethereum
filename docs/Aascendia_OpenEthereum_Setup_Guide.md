# Ascendia OpenEthereum Setup Guide

This guide provides step-by-step instructions for setting up Ascendia OpenEthereum on your server.

## Prerequisites

[OpenEthereum Container Update Guide](#openethereum-container-update-guide)

Before you begin, ensure that you have the following prerequisites:

- Access to a Linux server
- Docker installed on your server
- Basic knowledge of using the command line

## Installation Steps

Follow these steps to set up Ascendia OpenEthereum:

1. Create necessary directories:
   ```shell
   mkdir server server/data
   cd server/data/


2. Download the blockchain data backup::

    ```shell
    curl -s https://backup.airdao.io/blockchain.tgz | sudo tar zxpf -
    ```

4. Fetch the Ascendia chain specification `chain.json`:

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

9. Start Ascendia OpenEthereum as a Docker container:

    ```shell
    docker run --name "amb-node" -d -v /root/server/data/:/app -p 8545:8545 ghcr.io/ascendia-network/openethereum:v3.3.3-amb1.2.4 --config /app/parity_config.toml
    ```

	 **_Very Important:_** The only stable and up-to-date version is the one used in the documentation, which is currently openethereum:v3.3.3-amb1.2.4.

10. Check if the Docker container is running:

    ```shell
    docker ps
    ```

    You should see the `amb-node` container listed with status "Up."


# OpenEthereum Container Update Guide

## Overview
This guide provides instructions for updating the OpenEthereum container version and the `chain.json` file.


1. Stop the Running Container

First, stop your currently running OpenEthereum container

```shell
docker stop [container_name]
```

2. Update the Container Version

Pull the new version of the container from Docker hub or your registry.

```shell
docker pull [container_name]:v3.3.3-amb1.2.4
```

3. Update chain.json

Edit the chain.json file to include the necessary updates for the new OpenEthereum version.

```shell
curl -o chain.json https://chainspec.ambrosus.io/
```

4. Restart the Container

Finally, restart the container with the updated version.

```shell
docker start [container_name]
```
Notes

Replace [container_name] and [new_version_tag] with your specific container name and version tag.

