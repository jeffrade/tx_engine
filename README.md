# tx_engine

#### An example Rust application that consumes a CSV of transactions and outputs the client summaries.

This project strictly adheres to Cargo [`fmt`](https://github.com/rust-lang/rustfmt#running-cargo-fmt) and [`clippy`](https://github.com/rust-lang/rust-clippy#step-3-run-clippy).

## Build
```
$ cargo build
```

## Run
```
$ cargo run -- transactions.csv > accounts.csv
```