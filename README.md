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
- [ ] Aspiration Windows
- [ ] Iterative Deepening
- [ ] Killer Moves
- [ ] Minimax with Alpha-Beta pruning
- [ ] Null Move Heuristic
- [ ] Transposition Tables
- [ ] Work-stealing multi-threaded search

### [`hexe_core`]

- [x] Bitboard and square-to-piece map chess board representations
- [x] Lookup tables and magic Bitboards without runtime initialization

## License

Hexe is licensed under either of

- [Apache License (Version 2.0)][license-apache]

- [MIT License][license-mit]

at your choosing.

> **Note:** This project initially began on 2017-01-04 in a separate repository.
> This repo is a rewrite and expansion of that one.

[`hexe`]: https://docs.rs/hexe
[`hexe_core`]: https://docs.rs/hexe_core

[travis]:       https://travis-ci.org/hexe-rs/Hexe
[travis-badge]: https://travis-ci.org/hexe-rs/Hexe.svg?branch=master
[appv]:         https://ci.appveyor.com/project/nvzqz/hexe
[appv-badge]:   https://ci.appveyor.com/api/projects/status/github/hexe-rs/Hexe?svg=true

[license-apache]: https://github.com/hexe-rs/Hexe/blob/master/LICENSE-APACHE
[license-mit]: https://github.com/hexe-rs/Hexe/blob/master/LICENSE-MIT

[clarke-laws]: https://en.wikipedia.org/wiki/Clarke%27s_three_laws
