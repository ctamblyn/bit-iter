# `bit-iter`

Iterate over the bits set in a word.

A `BitIter` may be constructed from any integral value.

## Example

```rust
fn main() {
    use bit_iter::*;

    let x : u32 = 0x10001;

    for b in BitIter::from(x) {
        println!("Bit {} is set.", b);
    }
}
```

Output:

```text
Bit 0 is set.
Bit 16 is set.
```
