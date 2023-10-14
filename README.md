# lew

The Lightweight Editor for Web.

## Run example

Setup dependencies:

```shell
cargo install wasm-bindgen-cli cargo-make
```

Build wasm client examples:

```shell
cargo make examples -p release
```

Run examples server:

```shell
cargo make run
```


## Development notes

Check the project:

```shell script
cargo check --all-features --all-targets
```

Run all tests:

```shell script
cargo test --all-features --all-targets
```

Check and perform formatting:

```shell script
cargo make checkfmt
cargo make fmt
```

To run clippy, use the following command:

```shell script
cargo make clippy
```
