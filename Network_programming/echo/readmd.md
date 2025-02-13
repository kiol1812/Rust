# TCP echo server and client
refer to [server.rs](./server.rs).
```shell
rustc ./server.rs
./server.exe # or ./server
```
windows: [ncat/download](https://nmap.org/download.html#windows)
```shell
nc 127.0.0.1 8888 # linux
ncat 127.0.0.1 8888 # windows
```
---
refer to [client.rs](./client.rs)
```shell
rustc ./server.rs
./server.exe # or ./server
```
```shell
rustc ./client.rs
./client.exe # or ./server
```