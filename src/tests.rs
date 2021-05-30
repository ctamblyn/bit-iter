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
