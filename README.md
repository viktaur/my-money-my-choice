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

A fresh FRAME-based [Substrate](https://www.substrate.io/) node, ready for hacking :rocket:

### Setup

Please first check the latest information on getting starting with Substrate dependencies required to build this project [here](https://docs.substrate.io/main-docs/install/).

### Development Testing

To test while developing, without a full build (thus reduce time to results):

```sh
cargo t -p pallet-dex
cargo t -p pallet-dpos
cargo t -p pallet-voting
cargo t -p <other crates>
```

### Build

Build the node without launching it, with `release` optimizations:

```sh
cargo b -r
```

### Run

Build and launch the node, with `release` optimizations:

```sh
cargo r -r -- --dev
```

### CLI Docs

Once the project has been built, the following command can be used to explore all CLI arguments and subcommands:

```sh
./target/release/node-template -h
```
