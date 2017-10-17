#![feature(test)]
extern crate test;
extern crate hexe_core;

use test::{Bencher, black_box};
use hexe_core::color::Color;

#[bench]
fn bench_from_str(b: &mut Bencher) {
    static STRINGS: &[&str] = &[
        "white", "whitE", "whiTE", "whITE", "wHITE", "WHITE",
        "black", "blacK", "blaCK", "blACK", "bLACK", "BLACK",
    ];
    b.iter(|| {
        for &s in STRINGS {
            let _: Result<Color, _> = black_box(black_box(s).parse());
        }
    });
}
