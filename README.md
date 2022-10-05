# Wasm Examples

This repository contains different little examples for Rust to be compiled with the target ```--target wasm32-wasi``` 
and executed using the ```wasmtime``` runtime.

## Compile

Requirements:
- wasmtime 1.0.0
- wasm32-wasi target installed


```cargo build --bins --target wasm32-wasi```

## Execution

For each example its necessary different flags and configurations to have a correct execution.
### TCP echo server

```
# Using the wasmtime CLI
wasmtime run --tcplisten 127.0.0.1:8080 --wasm-features threads target/wasm32-wasi/debug/tcp-echo-server.wasm```