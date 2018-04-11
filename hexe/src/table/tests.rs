use super::*;

#[test]
fn new_zero() {
    let mut s: u16 = 0;

    for n in (0..4).map(|i| 1 << i) {
        let table = Table::new(n);
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
    for mut n in (0..4).map(|i| 1 << i) {
        let mut table = Table::new(n);
        assert_eq!(table.size_mb(), n);

        n = (n + 5) / 2;
        table.resize(n);
        assert_eq!(table.size_mb(), n.next_power_of_two());
    }
}

#[test]
fn is_aligned() {
    for mut n in 0..16 {
        let mut table = Table::new(n);
        assert!(table.0.is_aligned());

        table.resize((n + 5) / 2);
        assert!(table.0.is_aligned());
    }
}
