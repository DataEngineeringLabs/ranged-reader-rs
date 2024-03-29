# Ranged reader

[![test](https://github.com/DataEngineeringLabs/ranged-reader-rs/actions/workflows/test.yaml/badge.svg)](https://github.com/DataEngineeringLabs/ranged-reader-rs/actions/workflows/test.yaml)
[![codecov](https://codecov.io/gh/DataEngineeringLabs/ranged-reader-rs/branch/main/graph/badge.svg?token=AgyTF60R3D)](https://codecov.io/gh/DataEngineeringLabs/ranged-reader-rs)

Convert low-level APIs to read ranges of files into structs that implement
`Read + Seek` and `AsyncRead + AsyncSeek`. See
[parquet_s3_async.rs](tests/it/parquet_s3_async.rs) for an example of this API to
read parts of a large parquet file from s3 asynchronously.

### Rational

Blob storage https APIs offer the ability to read ranges of bytes from a single blob,
i.e. functions of the form

```rust
fn read_range_blocking(path: &str, start: usize, length: usize) -> Vec<u8>;
async fn read_range(path: &str, start: usize, length: usize) -> Vec<u8>;
```

together with its total size,

```rust
async fn length(path: &str) -> usize;
fn length(path: &str) -> usize;
```

These APIs are usually IO-bounded - they wait for network.

Some file formats (e.g. Apache Parquet, Apache Avro, Apache Arrow IPC) allow reading
parts of a file for filter and projection push down.

This crate offers 2 structs, `RangedReader` and `RangedStreamer` that implement
`Read + Seek` and `AsyncRead + AsyncSeek` respectively, to bridge the blob storage
APIs mentioned above to the traits used by most Rust APIs to read bytes.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
