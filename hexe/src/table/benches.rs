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
    table_new_1_mb, 1;
    table_new_4_mb, 4;
    table_new_8_mb, 8;
}
