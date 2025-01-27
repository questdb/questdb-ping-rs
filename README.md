# questdb-ping-rs

## Overview

Example code on how to check if a QuestDB instance is up and running using Rust.

The crate's `ping_questdb` function takes the same config string that is passed
to the ILP client. In other words, the same one passed to `Sender::from_conf` in
the [`questdb-rs`](https://docs.rs/questdb-rs/latest/questdb/) crate.

## Example

E.g. if you're sending ILP via:

```rust
Sender::from_conf("http::addr=localhost:9000;")
```

Check if the DB is up with:

```rust
use questdb-ping-rs::ping_questdb;

ping_questdb("http::addr=localhost:9000;")
```

See [`src/main.rs`](src/main.rs) for the full example.

## Dependencies

This crates depends on `ureq==2.9`. This is the same version (as of writing,
2025-01-27) used by the `questdb-rs` crate.

This dependency was chose in an effort to reduce the number of dependencies
that a client project would use.

