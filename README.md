# lew

The Lightweight Editor for Web.

## Run example

Setup dependencies:

```shell script
cargo install wasm-bindgen-cli
```

Build wasm client example:

```shell script
./examples/build.sh <example>
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

Enable autoformatting for IntelliJ IDEA with the Rust plugin:

`Settings -> Languages and Frameworks -> Rust -> Rustfmt -> Run rustfmt on Save`

Run clippy:

```shell script
cargo clippy --all-targets --all-features -- -D warnings
```

Setup git hook:

```shell script
cp .git-pre-push.sh .git/hooks/pre-push
```