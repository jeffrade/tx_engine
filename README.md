# tx_engine

#### An example Rust application that consumes a CSV of transactions and outputs client account summaries.

This project strictly adheres to Cargo [`fmt`](https://github.com/rust-lang/rustfmt#running-cargo-fmt) and [`clippy`](https://github.com/rust-lang/rust-clippy#step-3-run-clippy).

## Build
```
$ cargo build
```

## Run
```
$ cargo run -- transactions.csv > accounts.csv
```

#### IMPORTANT! Only if `errors.log` is empty or doesn't exist is it safe to assume accounts.csv is correct.

## TODO
 - Unit tests
 - Integration tests
 - Optimize code to increase memory and CPU performance
 - Introduce file chunking to consume very large files (e.g. 10 million rows)