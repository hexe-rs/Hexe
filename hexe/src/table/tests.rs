use super::*;

#[test]
fn new_zero() {
    let mut s: u16 = 0;

    for n in 0..16 {
        let table = Table::new(n, true);
        for cls in table.clusters() {
            for ent in cls.entries().iter() {
                s += ent.mv;
            }
        }
    }

    assert_eq!(s, 0);
}

#[test]
fn size_mb() {
    for mut n in 0..16 {
        let mut table = Table::new(n, true);
        assert_eq!(table.size_mb(), n);

        n = (n + 5) / 2;
        table.resize_exact(n);
        assert_eq!(table.size_mb(), n);

        table.resize(n);
        assert_eq!(table.size_mb(), n.next_power_of_two());
    }
}

#[test]
fn is_aligned() {
    for mut n in 0..16 {
        let mut table = Table::new(n, true);
        assert!(table.is_aligned());

        n = (n + 5) / 2;
        table.resize_exact(n);
        assert!(table.is_aligned());

        table.resize(n);
        assert!(table.is_aligned());
    }
}
