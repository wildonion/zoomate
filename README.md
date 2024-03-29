

distributed internet on ICP:

- zoomate node actors: p2p, tcp and grpc actor based proxy and workload-aware agent

- manit dashboard for monitoring nodes and balancers written in **Yew** and **Tauri**.

- hoopoe, the platform for social events

- ICP networking canisters

> current nginx password for `logs` dir: `zoomate@1234`

> refer to https://docs.cossacklabs.com/themis/installation/installation-from-packages/ if you don't want to build themis from source.

first clone the repo then install the followings:

```bash
wget http://archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2_amd64.deb
sudo dpkg -i libssl1.1_1.1.1f-1ubuntu2_amd64.deb
sudo apt update -y && sudo apt upgrade && sudo apt install -y libpq-dev pkg-config build-essential libudev-dev libssl-dev librust-openssl-dev
git clone https://github.com/cossacklabs/themis.git
cd themis
make
sudo make install
# install themis on MacOS M1
brew install libthemis
```

### protobuf setup

```bash
brew install protobuf # on MacOS
sudo apt install protobuf-compiler libssl-dev zlib1g-dev
```

### ICP canisters setup 

```bash
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
cd canisters && dfx new --type=rust wepn
```

compile `build.rs` codes which contains the `.proto` files in `proto` folder using ```cargo build``` command to generate the rust codes.

## 🥙 Usage

```bash
# run node + gRPC server
cd node && cargo run --bin zoomate -- --server 0.0.0.0 --port 50051
```

## 🪴 Setup and Run Yew based Tauri Manit Dashboard

```bash
# Yew based ui in Tauri
cargo install create-tauri-app
cargo install tauri-cli
cargo install trunk
cargo tauri dev
# only Yew
cargo install cargo-generate
cargo generate --git https://github.com/yewstack/yew-trunk-minimal-template ###### build a new yew app
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
trunk serve --port 9837
```
## 🧱 WIPs

- update crates, new rust edition, concetps spec
- hoopoe, icp contract, manit(yew,tauri,wasm), ed25519_with_aes_signing, zoomate.spec
- lib, acter, dp, redis4, raptor, tcpserver, misc dsl macros, bpf, zkp
- secure_session method
- compile to .so, .wasm