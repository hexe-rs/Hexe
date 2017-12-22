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

[![Build Status][travis-badge]][travis]
![LoC](https://tokei.rs/b1/github/hexe-rs/Hexe)

This project is split into two crates

- `hexe`: The chess engine itself.

- `hexe_core`: The chess engine's building blocks. This crate may be used by
other chess programs for ease of code reuse.

## Why Hexe?

"Hexe" is German for witch. It denotes the use of magic bitboards within this
project.

It also refers to Clarke's Third Law: "Any sufficiently advanced technology is
indistinguishable from magic."

Also, if she weighs the same as a duck... she's made of wood. And therefore...

## License

Hexe is licensed under either of

- [Apache License (Version 2.0)][license-apache]

- [MIT License][license-mit]

at your choosing.

[license-apache]: https://github.com/hexe-rs/Hexe/blob/master/LICENSE-APACHE
[license-mit]: https://github.com/hexe-rs/Hexe/blob/master/LICENSE-MIT

> **Note:** This project initially began on 2017-01-04 in a separate repository.
> This repo is a rewrite and expansion of that one.

[travis]:       https://travis-ci.org/hexe-rs/Hexe
[travis-badge]: https://travis-ci.org/hexe-rs/Hexe.svg?branch=master
