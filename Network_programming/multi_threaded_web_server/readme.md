# Multi-threaded Web Server
refer to [building a web server in rust part 2](https://youtu.be/1AamFJGAE8E?si=yCtsEJi_Lv8zUcY1)
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
<!-- ```toml
# Cargo.toml
...
[dependencies]
``` -->
```rust
// GET /sleep HTTP/1.1
thread::sleep(Duration::from_secs(5));
```
---
```shell
cargo run # under ./multi_threaded_web_server
```
test:
```plain
localhost:8080/sleep
```
```plain
localhost:8080
```