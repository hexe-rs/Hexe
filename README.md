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

[![Build Status][travis-badge]][travis]
![LoC](https://tokei.rs/b1/github/hexe-rs/Hexe)

This project is split into three crates

- **[bin]** `hexe_bin`: The runnable chess engine itself.

- **[lib]** [`hexe`]: The chess engine's guts and insides.

    - Specialized for Hexe's use cases.

- **[lib]** [`hexe_core`]: The chess engine's building blocks.

    - Supports `no_std` builds.

    - May be used by other chess programs for ease of code reuse.

## Why Hexe?

1. "Hexe" is German for witch. It denotes the use of magic bitboards within this
project.

2. It refers to [Clarke's Third Law][clarke-laws]: "Any sufficiently advanced
technology is indistinguishable from magic."

3. If she weighs the same as a duck... she's made of wood. And therefore...

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

[license-apache]: https://github.com/hexe-rs/Hexe/blob/master/LICENSE-APACHE
[license-mit]: https://github.com/hexe-rs/Hexe/blob/master/LICENSE-MIT

[clarke-laws]: https://en.wikipedia.org/wiki/Clarke%27s_three_laws
