# Ranged reader

Convert low-level APIs to read files in ranges into structs that implement `Read + Seek`.

### Rational

Some blob storage APIs offer the ability to read ranges of bytes from them, i.e. functions of the
form

```rust
async fn read_range(blob, start: usize, len: usize) -> Vec<u8>;
fn read_range_blocking(blob, start: usize, len: usize) -> Vec<u8>;
```

together with its total size, 

```rust
async fn length(blob) -> usize;
fn length(blob) -> usize;
```

These APIs are usually IO-bounded.

On the other end, some file formats (e.g. `parquet`) allow seeks to relevant parts of 
the file, e.g. for filter and projection push down.

Yet, many high-level APIs expect a `Read + Seek` API.

This crate offers 2 structs, `RangedReader` and `RangedStreamer` that implement
`Read + Seek` and `AsyncRead + AsyncSeek` respectively, expecting the APIs declared above.

Currently, only the sync API is implemented.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
