//! Iterate over the bits set in a word.
//!
//! A `BitIter` may be constructed from any integral value.
//!
//! A `BitIter` may be constructed from any integral value, and returns the positions of the `1`
//! bits in ascending order.
//!
//! `BitIter` implements `DoubleEndedIterator`, so you can iterate over the positions of the set
//! bits in descending order too.
//!
//! ## Example
//!
//! ```rust
//! # use bit_iter::*;
//! let x : u32 = 0x10001;
//!
//! for b in BitIter::from(x) {
//!     println!("Bit {} is set.", b);
//! }
//!
//! println!("In reverse order:");
//!
//! for b in BitIter::from(x).rev() {
//!     println!("Bit {} is set.", b);
//! }
//! ```
//!
//! Output:
//!
//! ```text
//! Bit 0 is set.
//! Bit 16 is set.
//! In reverse order:
//! Bit 16 is set.
//! Bit 0 is set.
//! ```

#![no_std]
#![doc(html_root_url = "https://docs.rs/bit-iter/1.1.0")]

use core::{
    iter::{ExactSizeIterator, FusedIterator},
    mem::size_of,
};

#[cfg(test)]
mod tests;

/// An iterator which returns the positions of the set bits in a word, in ascending order.
///
/// ## Examples
///
/// Construct a `BitIter` from an integer:
///
/// ```rust
/// # use bit_iter::*;
/// let mut iter = BitIter::from(0b10000001);
/// assert_eq!(iter.next(), Some(0usize));
/// assert_eq!(iter.next(), Some(7usize));
/// assert_eq!(iter.next(), None);
/// ```
///
/// Iterate over the bits in an integer in ascending order:
///
/// ```rust
/// # use bit_iter::*;
/// let v : Vec<usize> = BitIter::from(0b10000001).collect();
/// assert_eq!(v, vec![0, 7]);
/// ```
///
/// `BitIter` implements `DoubleEndedIterator`, so you can also get the set bit positions in
/// descending order:
///
/// ```rust
/// # use bit_iter::*;
/// let v : Vec<usize> = BitIter::from(0b10000001).rev().collect();
/// assert_eq!(v, vec![7, 0]);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct BitIter<T>(T);

macro_rules! iter_impl {
    ($($t:ty)*) => {$(
        /// `From` implementation for `BitIter`.
        impl From<$t> for BitIter<$t> {
            /// Construct a BitIter value.
            #[inline]
            fn from(value: $t) -> Self {
                Self(value)
            }
        }

        /// `Iterator` implementation for `BitIter`.
        impl Iterator for BitIter<$t> {
            type Item = usize;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if self.0 != 0 {
                    let trailing = self.0.trailing_zeros() as usize;
                    self.0 &= self.0.wrapping_sub(1);
                    Some(trailing)
                } else {
                    None
                }
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let sz = self.0.count_ones() as usize;
                (sz, Some(sz))
            }

            #[inline]
            fn count(self) -> usize {
                self.0.count_ones() as usize
            }

            #[inline]
            fn last(self) -> Option<Self::Item> {
                if self.0 != 0 {
                    Some(8 * size_of::<$t>() - 1 - self.0.leading_zeros() as usize)
                } else {
                    None
                }
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                let mut i = 0;
                while self.0 != 0 && i < n {
                    self.0 &= self.0.wrapping_sub(1);
                    i += 1;
                }
                self.next()
            }

            #[inline]
            fn fold<B, F>(mut self, init: B, mut f: F) -> B
            where
                F: FnMut(B, Self::Item) -> B
            {
                let mut accum = init;
                while self.0 != 0 {
                    accum = f(accum, self.0.trailing_zeros() as usize);
                    self.0 &= self.0.wrapping_sub(1);
                }
                accum
            }

            #[inline]
            fn max(self) -> Option<Self::Item> {
                self.last()
            }

            #[inline]
            fn min(self) -> Option<Self::Item> {
                if self.0 != 0 {
                    Some(self.0.trailing_zeros() as usize)
                } else {
                    None
                }
            }
        }

        /// `FusedIterator` implementation for `BitIter`.
        impl FusedIterator for BitIter<$t> {}

        /// `DoubleEndedIterator` implementation for `BitIter`.
        impl DoubleEndedIterator for BitIter<$t> {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                if self.0 != 0 {
                    let highest = 8 * size_of::<$t>() - 1 - self.0.leading_zeros() as usize;
                    self.0 ^= 1 as $t << highest;
                    Some(highest)
                } else {
                    None
                }
            }
        }

        /// `ExactSizeIterator` implementation for `BitIter`.
        impl ExactSizeIterator for BitIter<$t> {
            #[inline]
            fn len(&self) -> usize {
                self.0.count_ones() as usize
            }
        }
    )*}
}

iter_impl! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
