//! Iterate over the bits set in a word.
//!
//! A `BitIter` may be constructed from any integral value.
//!
//! ## Example
//!
//! ```rust
//! fn main() {
//!     use bit_iter::*;
//!
//!     let x : u32 = 0x10001;
//!
//!     for b in BitIter::from(x) {
//!         println!("Bit {} is set.", b);
//!     }
//! }
//! ```
//!
//! Output:
//!
//! ```text
//! Bit 0 is set.
//! Bit 16 is set.
//! ```

#![no_std]

#[cfg(test)]
mod tests;

pub struct BitIter<T>(T);

macro_rules! iter_impl {
    ($($t:ty)*) => {$(
        /// `From` implementation for `BitIter`.
        impl From<$t> for BitIter<$t> {
            /// Construct a BitIter value.
            fn from(value: $t) -> Self {
                Self(value)
            }
        }

        /// `Iterator` implementation for `BitIter`.
        impl Iterator for BitIter<$t> {
            type Item = usize;

            fn next(&mut self) -> Option<usize> {
                if self.0 != 0 {
                    let trailing = self.0.trailing_zeros() as usize;
                    self.0 &= self.0.wrapping_sub(1);
                    Some(trailing)
                } else {
                    None
                }
            }
        }
    )*}
}

iter_impl! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
