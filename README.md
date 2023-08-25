# NearMars-Way-to-The-Future

## Prerequisites

* [NEAR Wallet Account](wallet.testnet.near.org)
* [Rust Toolchain](https://docs.near.org/develop/prerequisites)
* [NEAR-CLI](https://docs.near.org/tools/near-cli#setup)
* [yarn](https://classic.yarnpkg.com/en/docs/install#mac-stable)

## Build
`yarn build`


# Get Rust in linux and MacOS
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env

# Add the wasm toolchain
rustup target add wasm32-unknown-unknown
