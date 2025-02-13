# Serde Server
refer to [rust-serde](../../Framework_Practise/serde_practise/readme.md).
```plain
serde_server/
├─Cargo.lock
├─Cargo.toml
├─src/
│   └─main.rs
└─target/...
```
- [cargo.toml](./Cargo.toml)
- [main.rs](./src/main.rs)
```toml
# Cargo.toml
...
[dependencies]
serde = "1.0.217"
serde_derive = "1.0.217"
serde_json = "1.0.138"
```
```shell
cargo run -- --server # under ./serde_server
```
```shell
cargo run -- --client # under ./serde_server
```