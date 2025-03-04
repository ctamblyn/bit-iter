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

## Minimum supported Rust version (MSRV) policy

`bit-iter`'s current minimum supported Rust version (MSRV) is **1.82.0**.

`bit-iter` is guaranteed to compile with that version.  It might also compile
with older versions, but that could change in a future patch release.

If the MSRV of `bit-iter` changes, that will be done in a _minor_ version
release (e.g. 1.3.x -> 1.4.0).
