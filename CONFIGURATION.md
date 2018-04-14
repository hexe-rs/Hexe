# Configuration

Compilation options can be configured via `RUSTFLAGS` or by
enabling/disabling project-specific features.

## Features

In your project's `Cargo.toml`:

```toml
[dependencies.hexe] # or hexe_core
version  = "0.0.5"
features = ["simd"]
```

Enables [SIMD]-accelerated operations **(nightly)**. See
[issue #4](https://github.com/hexe-rs/Hexe/issues/4) for more information.

Once SIMD is stable ([#48556](https://github.com/rust-lang/rust/issues/48556)),
this feature will be made a default. By opting out, Hexe can still compile using
previous `rustc` versions without SIMD support.

## Compiler Flags

Hexe may improve in performance if `rustc` is told to use features specific to
the compilation target.

### Target Feature

```sh
RUSTFLAGS="-C target-feature +$FEATURE"
```

Some features that may be worth using:

- `popcnt` **(`x86`, `x86_64`)**:

  Enables the hardware **population count** instruction instead of the slower
  software algorithm. This improves the performance of methods such as
  `Bitboard::len` and `PieceMap::len`.

### Target CPU

Another way of improving performance is by setting `target-cpu`:

```sh
RUSTFLAGS="-C target-cpu=native"
```

This will enable all `target-feature`s available for the CPU family.

[SIMD]: https://en.wikipedia.org/wiki/SIMD
