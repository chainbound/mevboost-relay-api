# mevoobst block finder

This library queries the MEVBoost relays for validator registrations in the current and next epoch,
and returns a list of blocks that are likely to be produced by mevboost-enabled validators.

You can import this as a crate in your Cargo project, or use the CLI tool to run queries manually.

## Use as a library

Add this to your `Cargo.toml`:

```toml
[dependencies]
mevboost-blocks = { git = "https://github.com/chainbound/mevboost-blocks" }
```

Then you can use it in your code:

```rust
use mevboost_blocks::?;
```

## Use as a CLI tool

```bash
cargo install --git https://github.com/chainbound/mevboost-blocks
mevboost-blocks --help
```

## License

MIT. Open source & free forever.
