# My money 💰, my choice 🙋
### Motivation
Most democratic nations are based on a system where their citizens choose between a limited number of parties (often two)
every four years and delegate their power to them. I believe this form of decision-making is not representative enough of the public opinion
and preferences, hence why I think there could be ways of democratising the system even more. Even though it is wise to rely on an educated group
of representatives that can engage in legislation and decision-making, citizens of a modern democracy should be able to directly
**choose where they want their money to be spent**. A potential transition to this new system becomes even more pertinent when one considers the
constant increase in public debt and cuts in funding of vital sectors happening in multiple states.

Through **quadratic voting**, we can enforce a more fair system where taxpayers can actually decide which public sectors
they want to finance, while being incentivised to spread it. To achieve this, I have implemented a pallet that can be used to construct
a Substrate runtime. This runtime could evolve into a parachain which could later be integrated in the Polkadot or Kusama
ecosystems, benefiting from the decentralisation and security provided.

### Specification
- Budget elections are meant to happen at the end of each fiscal year, but this is not a limitation. Budget elections are
  set a deadline when they are open (by the root account) and they need to be closed after, to avoid continuous polling. Any
  vote after the deadline will be invalidated and any citizen can close the election. There can only be one budget at a time.
- The budget is composed of 10 departments, named below.
- Citizens use voting credit to decide how much funding they want each department to have. They will need $x^2$ credits
  for each $x$ units of funding. The quadratic nature of the system encourages voters to spread their funding across multiple
  departments. Citizens can abstain from voting, but they cannot vote the same department twice. They also
  cannot change their votes once emitted. They need to allocate whole units of funding (non-fractional) and naturally, they
  end up with spare credit left (although they cannot be used for the next election).
- Citizens need to be registered before the budget election is open. Everyone receives 0 voting credit when registered and
  voting credit is given when the budget opens. Therefore, if a citizen registers after the election has open, they will
  still be able to do so, but they won't be able to vote due to a lack of voting credit.
- It is important to keep in mind that citizens are not deciding the total outlay, rather the ratio of each department
  respect to others.

#### Departments
- `Education 📚`
- `Employment 💼`
- `Healthcare 🏥`
- `Infrastructure 🚊`
- `Military 🪖`
- `Politics 🎌`
- `Repaying Public Debt 🫰`
- `Public Grants 🫂`
- `Science and Technology 🔬`
- `Social Security 👵`

### Potential expansion
- I started developing a UI using Svelte, but it's still very early stage. It would be great to interact with the
  system through a frontend calling the PolkadotJS API.
- To allow for better customisation and scalability, some properties like the departments and perhaps the quadratic
  function used should be generic.
- Additional mode where the voting credit given is proportional to their contributions through taxation. The quadratic
  nature of system would have an impact on both diversity of department choices and personal income. Whether this system
  would be more fair than conventional democratic means is out of my scope.

### Additional notes
- Terms "voting" and "funding" are usually used interchangeably, since casting a vote for a department using a certain
  amount of voting credit will translate into outlay funds sent to a department.
- "Budget" and "budget election" are also meant to represent the same thing: a decision-making system open for a limited
  period of time where citizens can cast votes representing the amount of funding each department should have.
---

## [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template)

A fresh [Substrate](https://substrate.io/) node, ready for hacking :rocket:

A standalone version of this template is available for each release of Polkadot in the [Substrate Developer Hub Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template/) repository.
The parachain template is generated directly at each Polkadot release branch from the [Node Template in Substrate](https://github.com/paritytech/substrate/tree/master/bin/node-template) upstream

It is usually best to use the stand-alone version to start a new project.
All bugs, suggestions, and feature requests should be made upstream in the [Substrate](https://github.com/paritytech/substrate/tree/master/bin/node-template) repository.

## Getting Started

Depending on your operating system and Rust version, there might be additional packages required to compile this template.
Check the [Install](https://docs.substrate.io/install/) instructions for your platform for the most common dependencies.
Alternatively, you can use one of the [alternative installation](#alternatives-installations) options.

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Embedded Docs

After you build the project, you can use the following command to explore its parameters and subcommands:

```sh
./target/release/node-template -h
```

You can generate and view the [Rust Docs](https://doc.rust-lang.org/cargo/commands/cargo-doc.html) for this template with this command:

```sh
cargo +nightly doc --open
```

### Single-Node Development Chain

The following command starts a single-node development chain that doesn't persist state:

```sh
./target/release/node-template --dev
```

To purge the development chain's state, run the following command:

```sh
./target/release/node-template purge-chain --dev
```

To start the development chain with detailed logging, run the following command:

```sh
RUST_BACKTRACE=1 ./target/release/node-template -ldebug --dev
```

Development chains:

- Maintain state in a `tmp` folder while the node is running.
- Use the **Alice** and **Bob** accounts as default validator authorities.
- Use the **Alice** account as the default `sudo` account.
- Are preconfigured with a genesis state (`/node/src/chain_spec.rs`) that includes several prefunded development accounts.


To persist chain state between runs, specify a base path by running a command similar to the following:

```sh
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/node-template --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```

### Connect with Polkadot-JS Apps Front-End

After you start the node template locally, you can interact with it using the hosted version of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) front-end by connecting to the local node endpoint.
A hosted version is also available on [IPFS (redirect) here](https://dotapps.io/) or [IPNS (direct) here](ipns://dotapps.io/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer).
You can also find the source code and instructions for hosting your own instance on the [polkadot-js/apps](https://github.com/polkadot-js/apps) repository.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, see [Simulate a network](https://docs.substrate.io/tutorials/build-a-blockchain/simulate-network/).

## Template Structure

A Substrate project such as this consists of a number of components that are spread across a few directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

- Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
  nodes in the network to communicate with one another.
- Consensus: Blockchains must have a way to come to [consensus](https://docs.substrate.io/fundamentals/consensus/) on the state of the network.
  Substrate makes it possible to supply custom consensus engines and also ships with several consensus mechanisms that have been built on top of [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
- RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory.
Take special note of the following:

- [`chain_spec.rs`](./node/src/chain_spec.rs): A [chain specification](https://docs.substrate.io/build/chain-spec/) is a source code file that defines a Substrate chain's initial (genesis) state.
  Chain specifications are useful for development and testing, and critical when architecting the launch of a production chain.
  Take note of the `development_config` and `testnet_genesis` functions,.
  These functions are used to define the genesis state for the local development chain configuration.
  These functions identify some [well-known accounts](https://docs.substrate.io/reference/command-line-tools/subkey/) and use them to configure the blockchain's initial state.
- [`service.rs`](./node/src/service.rs): This file defines the node implementation.
  Take note of the libraries that this file imports and the names of the functions it invokes.
  In particular, there are references to consensus-related topics, such as the [block finalization and forks](https://docs.substrate.io/fundamentals/consensus/#finalization-and-forks) and other [consensus mechanisms](https://docs.substrate.io/fundamentals/consensus/#default-consensus-models) such as Aura for block authoring and GRANDPA for finality.



### Runtime

In Substrate, the terms "runtime" and "state transition function" are analogous.
Both terms refer to the core logic of the blockchain that is responsible for validating blocks and executing the state changes they define.
The Substrate project in this repository uses [FRAME](https://docs.substrate.io/learn/runtime-development/#frame) to construct a blockchain runtime.
FRAME allows runtime developers to declare domain-specific logic in modules called "pallets".
At the heart of FRAME is a helpful [macro language](https://docs.substrate.io/reference/frame-macros/) that makes it easy to create pallets and flexibly compose them to create blockchains that can address [a variety of needs](https://substrate.io/ecosystem/projects/).

Review the [FRAME runtime implementation](./runtime/src/lib.rs) included in this template and note the following:

- This file configures several pallets to include in the runtime.
  Each pallet configuration is defined by a code block that begins with `impl $PALLET_NAME::Config for Runtime`.
- The pallets are composed into a single runtime by way of the [`construct_runtime!`](https://paritytech.github.io/substrate/master/frame_support/macro.construct_runtime.html) macro, which is part of the [core FRAME pallet library](https://docs.substrate.io/reference/frame-pallets/#system-pallets).

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with [the Substrate repository](https://github.com/paritytech/substrate/tree/master/frame) and a template pallet that is [defined in the `pallets`](./pallets/template/src/lib.rs) directory.

A FRAME pallet is comprised of a number of blockchain primitives, including:

- Storage: FRAME defines a rich set of powerful [storage abstractions](https://docs.substrate.io/build/runtime-storage/) that makes it easy to use Substrate's efficient key-value database to manage the evolving state of a blockchain.
- Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched) from outside of the runtime in order to update its state.
- Events: Substrate uses [events](https://docs.substrate.io/build/events-and-errors/) to notify users of significant state changes.
- Errors: When a dispatchable fails, it returns an error.

Each pallet has its own `Config` trait which serves as a configuration interface to generically define the types and parameters it depends on.

## Alternatives Installations

Instead of installing dependencies and building this source directly, consider the following alternatives.

### Nix

Install [nix](https://nixos.org/) and
[nix-direnv](https://github.com/nix-community/nix-direnv) for a fully plug-and-play
experience for setting up the development environment.
To get all the correct dependencies, activate direnv `direnv allow`.

### Docker

Please follow the [Substrate Docker instructions here](https://github.com/paritytech/substrate/blob/master/docker/README.md) to build the Docker container with the Substrate Node Template binary.
