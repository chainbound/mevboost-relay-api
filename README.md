# mevboost relay api

> This crate implements the [PBS Relay API](https://flashbots.github.io/relay-specs/#/) specs in Rust.
> You can import this in your Cargo project, or use the CLI tool to run queries manually.

## Use as a library

Add this to your `Cargo.toml`:

```toml
[dependencies]
mevboost-relay-api = { git = "https://github.com/chainbound/mevboost-relay-api" }
```

Then you can use it in your code:

```rust
#[tokio::main]
pub async fn main() {
    let client = mevboost_relay_api::Client::default();
    let response = client.get_validators_for_current_and_next_epoch("flashbots").await.unwrap();
    println!("{:?}", response);
}
```

You can see all available functionality in the [library documentation](./crates/mevboost-relay-api/README.md) file.

## Use as a CLI tool

> **Warning**
> This is not yet implemented.

```shell
cargo install --git https://github.com/chainbound/mevboost-relay-api
mevboost-relay-api --help
```

## License

MIT.

## Contributions

Contributions are welcome and encouraged. Please open an issue or submit a pull request.
