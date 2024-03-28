# NH-core
Implementation of a node for the **New Horizen Proof Verification Layer**.

It is based on the [Substrate](https://substrate.io/) framework.

> [!IMPORTANT]
> ***NH-core*** is currently in an early **testnet** stage.
> The plan for going live on Mainnet will be communicated later.
> For more information see [horizen.io](https://www.horizen.io/).

## Building and running

To build the client from source, clone this repository and run the following commands from the root of the project:

```bash
git checkout <latest tagged release>
cargo build --release
```

It is possible to run tests with:

```bash
cargo test
```

To run a local dev node:
```bash
cd target/release
./nh-node --dev
```

The client will run a chain with a single validator (Alice) and start producing blocks.

```
2024-03-28 11:49:08 New Horizen Mainchain Node
2024-03-28 11:49:08 ✌️  version 0.1.0-deda6a0980c
2024-03-28 11:49:08 ❤️  by Horizen Labs <admin@horizenlabs.io>, 2024-2024
2024-03-28 11:49:08 📋 Chain specification: Development
2024-03-28 11:49:08 🏷  Node name: Alice
2024-03-28 11:49:08 👤 Role: AUTHORITY
2024-03-28 11:49:08 💾 Database: RocksDb at /tmp/substrateVqTiy0/chains/dev/db/full
2024-03-28 11:49:08 [0] 💸 generated 1 npos voters, 1 from validators and 0 nominators
2024-03-28 11:49:08 [0] 💸 generated 1 npos targets
2024-03-28 11:49:08 🔨 Initializing Genesis block/state (state: 0x271d…3d28, header-hash: 0x1b7e…5b3e)
2024-03-28 11:49:08 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.
2024-03-28 11:49:09 Using default protocol ID "sup" because none is configured in the chain specs
2024-03-28 11:49:09 🏷  Local node identity is: 12D3KooWRRRVCzJNGdhAfMW4fzpA3HwQb498uecyp8NsAaLxCuhq
2024-03-28 11:49:09 💻 Operating system: linux
2024-03-28 11:49:09 💻 CPU architecture: x86_64
2024-03-28 11:49:09 💻 Target environment: gnu
2024-03-28 11:49:09 💻 CPU: 11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz
2024-03-28 11:49:09 💻 CPU cores: 8
2024-03-28 11:49:09 💻 Memory: 15856MB
2024-03-28 11:49:09 💻 Kernel: 5.15.146.1-microsoft-standard-WSL2
2024-03-28 11:49:09 💻 Linux distribution: Ubuntu 20.04.6 LTS
2024-03-28 11:49:09 💻 Virtual machine: yes
2024-03-28 11:49:09 📦 Highest known block at #0
2024-03-28 11:49:09 〽️ Prometheus exporter started at 127.0.0.1:9615
2024-03-28 11:49:09 Running JSON-RPC server: addr=127.0.0.1:9944, allowed origins=["*"]
2024-03-28 11:49:12 🙌 Starting consensus session on top of parent 0x1b7ebdeb01f5506a6bcbe83277477696194baf2be903617258113bdc9b385b3e
2024-03-28 11:49:12 🎁 Prepared block for proposing at 1 (0 ms) [hash: 0x831921d2e2c853bd5feedbef5885a7de7c0622668fb49a2a6aba9c2611afcbe6; parent_hash: 0x1b7e…5b3e; extrinsics (1): [0xb95e…ab06]
2024-03-28 11:49:12 🔖 Pre-sealed block for proposal at 1. Hash now 0x89e3b0332a1fc1ba984aba6699872301e1f22e0efa7e80eeb76c3af9b711c6c4, previously 0x831921d2e2c853bd5feedbef5885a7de7c0622668fb49a2a6aba9c2611afcbe6.
2024-03-28 11:49:12 ✨ Imported #1 (0x89e3…c6c4)
2024-03-28 11:49:14 💤 Idle (0 peers), best: #1 (0x89e3…c6c4), finalized #1 (0x89e3…c6c4), ⬇ 0 ⬆ 0
2024-03-28 11:49:18 🙌 Starting consensus session on top of parent 0x89e3b0332a1fc1ba984aba6699872301e1f22e0efa7e80eeb76c3af9b711c6c4
2024-03-28 11:49:18 🎁 Prepared block for proposing at 2 (0 ms) [hash: 0xea3c4edc3223623ccbbfa6871e05e3a1b8b6b8a9ed0b97de37fde441d9860c78; parent_hash: 0x89e3…c6c4; extrinsics (1): [0x2e13…7d95]
2024-03-28 11:49:18 🔖 Pre-sealed block for proposal at 2. Hash now 0x8226727507239e061f089d102f346e0e6c285a7d73a1dce3e000196f1dbedf51, previously 0xea3c4edc3223623ccbbfa6871e05e3a1b8b6b8a9ed0b97de37fde441d9860c78.
2024-03-28 11:49:18 ✨ Imported #2 (0x8226…df51)
2024-03-28 11:49:19 💤 Idle (0 peers), best: #2 (0x8226…df51), finalized #1 (0x89e3…c6c4), ⬇ 0 ⬆ 0
2024-03-28 11:49:24 🙌 Starting consensus session on top of parent 0x8226727507239e061f089d102f346e0e6c285a7d73a1dce3e000196f1dbedf51
2024-03-28 11:49:24 🎁 Prepared block for proposing at 3 (0 ms) [hash: 0xfefa63f0d35437f9f66842f2b165af805aeb6b3e8afe41d0c9f652c95b654184; parent_hash: 0x8226…df51; extrinsics (1): [0x5d14…4d83]
2024-03-28 11:49:24 🔖 Pre-sealed block for proposal at 3. Hash now 0xf308498cc798d66c6cbeeddffbb70d030b30362e16d305d812e21cc4edb5e8ac, previously 0xfefa63f0d35437f9f66842f2b165af805aeb6b3e8afe41d0c9f652c95b654184.
2024-03-28 11:49:24 ✨ Imported #3 (0xf308…e8ac)
2024-03-28 11:49:24 💤 Idle (0 peers), best: #3 (0xf308…e8ac), finalized #2 (0x8226…df51), ⬇ 0 ⬆ 0
```

## Docker
NH-core includes some Docker files for building the client and running one or more nodes locally.
For more information, see [docker/README.md](docker/README.md]).

## License
NH-core as a whole is released under the [GPL 3.0 license](LICENSE-GPL3). This is mostly due to the fact that the proof verification implemented by the `settlement-fflonk` pallet is based on GPL 3.0 software.

For this reason, all the crates that include such dependency are GPL 3.0 licensed:
- `pallet-settlement-fflonk`
- `nh-runtime`
- `mainchain`

The remaining crates, which are independent of the FFLONK verifier implementation, are released under the [APACHE 2.0 license](LICENSE-APACHE2):
- `pallet-poe`
- `hp-poe`
- `proof-of-existence-rpc`
- `proof-of-existence-rpc-runtime-api`