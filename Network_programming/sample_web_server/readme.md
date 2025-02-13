# Sample Web Server
refer to [building a web server in rust part 3]()
```plain
multi_threaded_web_server/
├─Cargo.lock
├─Cargo.toml
├─src/
│   ├─bin/
│   │   └─main.rs
│   └─lib.rs
├─pages/
│   ├─index.html
│   └─404.html
└─target/...
```
- [main.rs](./src/bin/main.rs)
- [lib.rs](./src/lib.rs)
---
```shell
cargo run # under ./sample_web_server
```