# Serde Practise
refer to [serde overview](https://serde.rs/) or [magiclen.org/rust-serde](https://magiclen.org/rust-serde/).  
also see [serde_practise](../../Framework_Practise/serde_practise/readme.md).
<!-- also see [serde-custom](../serde_custom/readme.md). -->
```plain
serde_practise/
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
serde_yaml = "0.9.34"
```
```shell
cargo run # under ./serde_practise
```