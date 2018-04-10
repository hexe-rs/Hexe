<p align="center">
    <a href="https://github.com/hexe-rs/Hexe/">
    <img
        src="https://raw.githubusercontent.com/hexe-rs/Hexe/assets/Icon.png"
        alt="Hexe"
        width="250"
    >
    </a>
    <br>
    A pure <a href="https://www.rust-lang.org">Rust</a> chess engine
</p>

----

<p align="center"><em>(pronounced "Hekseh")</em></p>

[![Travis Status][travis-badge]][travis]
[![AppVeyor Status][appv-badge]][appv]
![LoC](https://tokei.rs/b1/github/hexe-rs/Hexe)

## What is Hexe?

Hexe is an open-source chess engine written in Rust. It is written primarily as
a library, with an executable that directly utilizes that library. This project
is split into two packages (crates):

- **[ lib + bin ]** [`hexe`]: The chess engine itself

    - Specialized for Hexe's use cases

    - Contains all code used to run the executable

- **[ lib ]** [`hexe_core`]: The chess engine's building blocks

    - Supports `no_std` builds

    - May be used by other chess programs for ease of code reuse

## Why "Hexe"?

1. "Hexe" is German for witch. It denotes the use of magic bitboards within this
project.

2. It refers to [Clarke's Third Law][clarke-laws]: "Any sufficiently advanced
technology is indistinguishable from magic."

3. If she weighs the same as a duck... she's made of wood. And therefore...

## Features

The goal of this project is to have the following features:

### [`hexe`]

- [x] UCI compatibility
- [x] Work-stealing multi-threaded search
- [ ] Aspiration Windows
- [ ] Iterative Deepening
- [ ] Killer Moves
- [ ] Minimax with Alpha-Beta pruning
- [ ] Null Move Heuristic
- [ ] [SIMD] parallelism (see [#4])
- [ ] Transposition Tables

### [`hexe_core`]

- [x] Bitboard and square-to-piece map chess board representations
- [x] Lookup tables and magic Bitboards without runtime initialization
- [ ] [SIMD] parallelism (see [#4])

### Cross-Platform Compatibility

Hexe is written to be available for the main platforms that Rust compiles to.
`hexe` and `hexe_core` are both automatically tested—separately—against all
[Tier 1 platforms][tier1]. As of this writing, they are:

| Platform   | Version      | Bits  |
| :--------- | :----------- | :---- |
| macOS      | 10.7+, Lion+ | 32/64 |
| MinGw/MSVC | Windows 7+   | 32/64 |
| Linux      | 2.6.18+      | 32/64 |

`hexe_core` is designed to not require [the Rust standard library][std]. Because
of this, it is compatible with all platforms that stable Rust compiles to.

### Cross-Language Compatibility

Hexe wrappers are currently available in the following languages:

- Swift
  - [Hexe.swift](https://github.com/hexe-rs/Hexe.swift/)

## License

Hexe is licensed under either of

- [Apache License (Version 2.0)][license-apache]

- [MIT License][license-mit]

at your choosing.

[#4]: https://github.com/hexe-rs/Hexe/issues/4

[`hexe`]:      https://docs.rs/hexe
[`hexe_core`]: https://docs.rs/hexe_core

[travis]:       https://travis-ci.org/hexe-rs/Hexe
[travis-badge]: https://travis-ci.org/hexe-rs/Hexe.svg?branch=master
[appv]:         https://ci.appveyor.com/project/nvzqz/hexe
[appv-badge]:   https://ci.appveyor.com/api/projects/status/github/hexe-rs/Hexe?svg=true

[license-apache]: https://github.com/hexe-rs/Hexe/blob/master/LICENSE-APACHE
[license-mit]: https://github.com/hexe-rs/Hexe/blob/master/LICENSE-MIT

[SIMD]:        https://en.wikipedia.org/wiki/SIMD
[std]:         https://doc.rust-lang.org/std/
[tier1]:       https://forge.rust-lang.org/platform-support.html#tier-1
[clarke-laws]: https://en.wikipedia.org/wiki/Clarke%27s_three_laws
