use super::*;

#[test]
fn zero_value_handled_correctly() {
    let mut iter = BitIter::from(0u8);
    assert_eq!(iter.next(), None);
}

#[test]
fn non_zero_unsigned_value_handled_correctly() {
    let mut iter = BitIter::from(0x80010011u32);
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(16));
    assert_eq!(iter.next(), Some(31));
    assert_eq!(iter.next(), None);
}

#[test]
fn non_zero_signed_value_handled_correctly() {
    let mut iter = BitIter::from(0x80010011u32 as i32);
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(16));
    assert_eq!(iter.next(), Some(31));
    assert_eq!(iter.next(), None);
}

#[test]
fn size_hint_works() {
    let iter = BitIter::from(0x25);
    assert_eq!(iter.size_hint(), (3, Some(3)));

    let iter = BitIter::from(1);
    assert_eq!(iter.size_hint(), (1, Some(1)));

    let iter = BitIter::from(0);
    assert_eq!(iter.size_hint(), (0, Some(0)));
}

#[test]
fn count_works() {
    let iter = BitIter::from(0x25);
    assert_eq!(iter.count(), 3);

    let iter = BitIter::from(1);
    assert_eq!(iter.count(), 1);

    let iter = BitIter::from(0);
    assert_eq!(iter.count(), 0);
}

#[test]
fn last_works() {
    let iter = BitIter::from(0x25);
    assert_eq!(iter.last(), Some(5usize));

    let iter = BitIter::from(1);
    assert_eq!(iter.last(), Some(0usize));

    let iter = BitIter::from(0);
    assert_eq!(iter.last(), None);
}

#[test]
fn nth_works() {
    let mut iter = BitIter::from(0x5f);
    assert_eq!(iter.nth(3), Some(3usize));
    assert_eq!(iter.nth(3), None);
}

#[test]
fn max_works() {
    let iter = BitIter::from(0x25);
    assert_eq!(iter.max(), Some(5usize));

    let iter = BitIter::from(1);
    assert_eq!(iter.max(), Some(0usize));

    let iter = BitIter::from(0);
    assert_eq!(iter.max(), None);
}

#[test]
fn min_works() {
    let iter = BitIter::from(0xa4);
    assert_eq!(iter.min(), Some(2usize));

    let iter = BitIter::from(0x80);
    assert_eq!(iter.min(), Some(7usize));

    let iter = BitIter::from(0);
    assert_eq!(iter.min(), None);
}

#[test]
fn can_iterate_in_reverse_order() {
    let mut iter = BitIter::from(0x80010011u32).rev();
    assert_eq!(iter.next(), Some(31));
    assert_eq!(iter.next(), Some(16));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);
}
