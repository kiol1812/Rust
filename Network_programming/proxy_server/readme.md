# Proxy Server
```plain
proxy_server/
├─Cargo.lock
├─Cargo.toml
├─src/
│   └─main.rs
└─target/...
```
- [main.rs](./src/main.rs)
```toml
# Cargo.toml
...
[dependencies]
hyper = { version = "0.14.25", features = ["full"] }
tokio = { version = "1.26.0", features = ["full"] }
tower = { version = "0.4", features = ["full"] }
modsecurity = "1.0.0"
```
---
note that you must install [libmodsecurity](https://crates.io/crates/modsecurity-rs).
```shell
cargo run # under ./proxy_server
```
e.g. if http server is running on localhost:8080
```shell
curl --proxy http://127.0.0.1:8000 http://127.0.0.1:8080
```