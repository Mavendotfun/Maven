# Maven

<div align="center">
  <img src="./assets/maven1500x500.png" alt="Maven Banner" width="100%" />
</div>

<div align="center">

 [Website](https://elizaos.github.io/eliza/)

</div>

## Introduction

Maven is revolutionizing the way on-chain agents access and utilizes computing power. Maven is providing a distributed GPU grid that enables anyone to rent compute power without the usual overhead, excessive fees, or vendor lock-in.

Maven is tackling the GPU shortage in the market, providing everyone with easy access to the necessary hardware. By leveraging underutilized hardware like gaming PCs, mining rigs, and MacBooks, Maven offers users to use $MAVEN tokens to build & deploy their own agents.

## Key Features
- Distributed GPU Grid
- No Vendor Lock-in
- Tokenized System
- Support for On-Chain Agents
- Reduced Overhead and Fees

 ## Getting Started
 
Maven is a platform for running AI inference workloadsopen in new window. Currently, the best way to run inference on the Maven network is through our CLI. We'll start with a simple hello world program and afterwards build on these concepts to create a fully working endpoint.

## Installing The Maven CLI

To install the Maven CLI globally, use the following command:

```
npm install -g @maven/cli
```
Verify the installation by running:
```
maven --version
```
```
  __  __     __      ________ _   _ 
 |  \/  |   /\ \    / /  ____| \ | |
 | \  / |  /  \ \  / /| |__  |  \| |
 | |\/| | / /\ \ \/ / |  __| | . ` |
 | |  | |/ ____ \  /  | |____| |\  |
 |_|  |_/_/    \_\/   |______|_| \_|

1.0.0
```
## Your Maven Wallet
When you first run the Nosana CLI, a new keypair is generated for you in ~/.maven/.maven_key.json, creating a new wallet. In order to run Maven jobs, you need to have some MAVEN in this wallet. You can purchase MAVEN on Raydium or swap in your Solana wallet.

Next, run Maven address, and the terminal will print out your address (the public key). If you need access to your private keys, they can be found in the local folder on your machine. However, for security reasons, we strongly advise against replacing this file.

Once you have MAVEN, send some to the address logged from the Maven address command. Now you're ready to start running jobs on the Maven network!

## How To Write A Job

# Overview

Nosana Jobs: Each job on the Nosana network is essentially an AI inference task that runs from a Docker container. The jobs are designed to utilize GPU resources efficiently.

Nosana Nodes: Jobs run on different Nosana nodes, which are represented as Solana addresses in the Nosana explorer. Each node contributes its GPU resources to execute the jobs.

GPU Markets: Different types of GPUs available in the network are categorized into various markets. Each market supports specific job requirements based on the type of GPU.


                                    



