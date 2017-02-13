#![feature(fixed_size_array)]
#![no_std]

extern crate typenum;

pub use array::Array;
pub use array_length::ArrayLength;

mod array;
mod array_length;
mod implementations;

#[cfg(test)]
mod tests {
    use super::{Array, ArrayLength};
    use typenum::consts;

    fn array<L>(a: Array<u32, L>) -> Array<u32, L> where L: ArrayLength<u32> { a }

    #[test]
    fn it_works() {
        let a = array :: <consts::U3> ([1, 2, 3]);
        
        assert_eq!(a, [1, 2, 3]);
    }
}