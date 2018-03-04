# Parrallel PI calculation

To build an optimized version with AVX2 support, compile the project as follows:

```
RUSTFLAGS='-C target-feature=avx2' cargo build --release
```
