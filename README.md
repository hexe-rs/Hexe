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

<div align="center">
    <a href="https://travis-ci.org/hexe-rs/Hexe">
        <img src="https://travis-ci.org/hexe-rs/Hexe.svg?branch=master" alt="Travis Status">
    </a>
    <a href="hhttps://ci.appveyor.com/project/nvzqz/hexe">
        <img src="https://ci.appveyor.com/api/projects/status/github/hexe-rs/Hexe?svg=true" alt="AppVeyor Status">
    </a>
    <img src="https://tokei.rs/b1/github/hexe-rs/Hexe" alt="Lines of code">
    <br>
    <a href="https://www.patreon.com/nvzqz">
        <img src="https://c5.patreon.com/external/logo/become_a_patron_button.png" alt="Become a Patron!" height="35">
    </a>
    <a href="https://www.paypal.me/nvzqz">
        <img src="https://buymecoffee.intm.org/img/button-paypal-white.png" alt="Buy me a coffee" height="35">
    </a>
</div>

## What is Hexe?

Hexe is an open-source chess engine written in Rust. It is written primarily as
a library, with a separate executable CLI frontend. This project is split into
three packages (crates):

- **[ bin ]** `hexe_bin`

  The CLI frontend for Hexe that actually executes the chess engine.

- **[ lib ]** [`hexe` (documentation)][hexe]

  The chess engine itself. All code is heavily opinionated and specialized for
  Hexe's use cases.

- **[ lib ]** [`hexe_core` (documentation)][hexe_core]

  The chess engine's building blocks. It can be used easily by other chess
  programs and engines for better code reuse and efficiency.

## Why "Hexe"?

1. "Hexe" means "witch" in German, denoting the use of
   [magic bitboards](https://www.chessprogramming.org/Magic_Bitboards) within
   this project.

   "Magic bitboards" refers to a very fast [perfect hash function][phf] used for
   indexing into a large precomputed table of bishop and rook attacks.

2. [Clarke's Third Law](https://en.wikipedia.org/wiki/Clarke%27s_three_laws):

   "Any sufficiently advanced technology is indistinguishable from magic."

3. If she weighs the same as a duck... she's made of wood. And therefore...

## Configuration

See [`CONFIGURATION.md`](https://github.com/hexe-rs/Hexe/blob/master/CONFIGURATION.md).

## Features

The goal of this project is to have the following features:

### [`hexe`][hexe]

- [x] UCI compatibility
- [x] [Work stealing](https://en.wikipedia.org/wiki/Work_stealing)
      multi-threaded search
- [ ] Aspiration Windows
- [ ] Iterative Deepening
- [ ] Killer Moves
- [ ] Minimax with Alpha-Beta pruning
- [ ] Null Move Heuristic
- [ ] [SIMD] parallelism (see [#4])
- [ ] Transposition Tables

### [`hexe_core`][hexe_core]

- [x] Bitboard and square-to-piece map chess board representations
- [x] Lookup tables
  - [x] Magic bitboards without runtime initialization
  - [x] Usually aligned to common cache line size (64 bytes)
- [x] Optional dependency on the Rust standard library or `libc`
- [ ] [SIMD] parallelism (see [#4])

## Compatibility

Hexe is automatically tested against a minimum Rust version: 1.25.0.

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

[hexe]: https://docs.rs/hexe
[hexe_core]: https://docs.rs/hexe_core

[travis]:       https://travis-ci.org/hexe-rs/Hexe
[travis-badge]: https://travis-ci.org/hexe-rs/Hexe.svg?branch=master
[appv]:         https://ci.appveyor.com/project/nvzqz/hexe
[appv-badge]:   https://ci.appveyor.com/api/projects/status/github/hexe-rs/Hexe?svg=true

[license-apache]: https://github.com/hexe-rs/Hexe/blob/master/LICENSE-APACHE
[license-mit]:    https://github.com/hexe-rs/Hexe/blob/master/LICENSE-MIT

[SIMD]:  https://en.wikipedia.org/wiki/SIMD
[phf]:   https://en.wikipedia.org/wiki/Perfect_hash_function
[std]:   https://doc.rust-lang.org/std/
[tier1]: https://forge.rust-lang.org/platform-support.html#tier-1
