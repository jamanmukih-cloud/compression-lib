# Compression Library 🗜️

Fast compression with LZ4, Zstd, and Brotli.

## Benchmarks

| Algorithm | Ratio | Speed (compress) | Speed (decompress) |
|-----------|-------|-------------------|---------------------|
| LZ4 | 2.1× | 750 MB/s | 4000 MB/s |
| Zstd | 3.5× | 500 MB/s | 1500 MB/s |
| Brotli | 4.2× | 100 MB/s | 400 MB/s |

## Quick Start

```rust
let compressed = compress(input, Algorithm::Zstd)?;
let decompressed = decompress(&compressed)?;
```

## License

BSD-2