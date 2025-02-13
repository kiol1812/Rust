# Single Threaded Web Server
refer to [building a web server in rust part 1](https://youtu.be/BHxmWTVFWxQ?si=k-PJk7VJMiqcTFE4).
```plain
single_threaded_web_server/
├─Cargo.lock
├─Cargo.toml
├─src/
│   └─main.rs
├─pages/
│   ├─index.html
│   └─404.html
└─target/...
```
- [main.rs](./src/main.rs)
<!-- ```toml
# Cargo.toml
...
[dependencies]
``` -->
```shell
cargo run # under ./single_threaded_web_server
```