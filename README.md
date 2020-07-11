<div align="center">

  <h1><code>substrate-stencil</code></h1>

  <strong>A template for kick starting a Rust and Blockchain project using <a href="https://github.com/paritytech/substrate">Substrate</a>.</strong>

  <h3>
    <a href="https://substrate.dev/">Docs</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

</div>

## Features

This template includes the minimum required components to start a PoS testnet, inspired by [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template).

* Consensus related pallets: Babe & GRANDPA
* Staking related pallets: staking, session, authorship, im-online, offences, utility
* Governance related pallets: collective, membership, elections-phragmen, democracy, treasure

## 🚴 Usage

### 📦 Installation

Install Rust and Substrate dependencies:

```
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

Install [cargo generate](https://github.com/ashleygwilliams/cargo-generate):

```
cargo install cargo-generate
```


### 🛠️ Generate & Build

Generate your project based the template:

```
cargo generate --git https://github.com/kaichaosun/substrate-stencil.git --name my-project

cd my-project
```

Build source code:

```bash
cargo build --release
```

### 🏃 Run

Purge any existing developer chain state:

```bash
./target/release/my-project purge-chain --dev
```

Start a development chain with:

```bash
./target/release/node-template --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

### 🕸️ Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a local testnet with two validator nodes for Alice and Bob, who are the initial authorities of the genesis chain that have been endowed with testnet units.

Optionally, give each node a name and expose them so they are listed on the Polkadot [telemetry site](https://telemetry.polkadot.io/#/Local%20Testnet).

You'll need two terminal windows open.

We'll start Alice's substrate node first on default TCP port 30333 with her chain database stored locally at `/tmp/alice`. The bootnode ID of her node is `QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR`, which is generated from the `--node-key` value that we specify below:

```bash
cargo run -- \
  --base-path /tmp/alice \
  --chain=local \
  --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator
```

In the second terminal, we'll start Bob's substrate node on a different TCP port of 30334, and with his chain database stored locally at `/tmp/bob`. We'll specify a value for the `--bootnodes` option that will connect his node to Alice's bootnode ID on TCP port 30333:

```bash
cargo run -- \
  --base-path /tmp/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR \
  --chain=local \
  --bob \
  --port 30334 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator
```

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.

### 🐳 Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can also replace the default command (`cargo build --release && ./target/release/node-template --dev --ws-external`) by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```
