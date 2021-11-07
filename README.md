# bit-iter

![Test results](https://github.com/ctamblyn/bit-iter/actions/workflows/quickstart.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/bit-iter)](https://crates.io/crates/bit-iter)
[![Documentation](https://docs.rs/bit-iter/badge.svg)](https://docs.rs/bit-iter)

Iterate forwards or backwards over the positions of bits set in a word.

A `BitIter` may be constructed from any integral value, and returns the
positions of the `1` bits in ascending order.

`BitIter` implements `DoubleEndedIterator`, so you can iterate over the
positions of the set bits in descending order too.

## Example

```rust
fn main() {
    use bit_iter::*;

    let x : u32 = 0x10001;

    for b in BitIter::from(x) {
        println!("Bit {} is set.", b);
    }

    println!("In reverse order:");

    for b in BitIter::from(x).rev() {
        println!("Bit {} is set.", b);
    }
}
```

Output:

```text
Bit 0 is set.
Bit 16 is set.
In reverse order:
Bit 16 is set.
Bit 0 is set.
```
