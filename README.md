# lew

The Lightweight Editor for Web.

## Run example

Setup dependencies:

```shell script
cargo install wasm-bindgen-cli
```

Build wasm client example:

```shell script
./examples/build.sh <example> release
```

Run example server:

```shell script
cargo run --example server
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
cargo +nightly fmt -- --check
cargo +nightly fmt
```

To enable autoformatting for IntelliJ IDEA with the Rust plugin:

`File -> Settings -> Languages & Frameworks -> Rust -> Rustfmt, check "Run rustfmt on Save"`

To run clippy, use the following command:

```shell script
cargo clippy --all-targets --all-features -- -D warnings
```

To setup git hook, use the following command:

```shell script
cp .git-pre-push.sh .git/hooks/pre-push
```