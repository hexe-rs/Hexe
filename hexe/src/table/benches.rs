use super::*;
use test::{Bencher, black_box};

macro_rules! impl_benches {
    ($($bench:ident, $num:expr;)+) => { $(
        #[bench]
        fn $bench(b: &mut Bencher) {
            b.iter(|| {
                let mut table = Table::new(black_box($num));
                black_box(&mut table);
            });
        }
    )+ }
}

impl_benches! {
    table_new_01_mb, 1;
    table_new_04_mb, 4;
    table_new_16_mb, 16;
}
