extern crate num_traits;
extern crate typenum;
extern crate generic_array;
extern crate bit_vec;
extern crate bit_array;

use std::ops::{Add, Div, Sub};
use num_traits::Float;
use typenum::*;
use generic_array::ArrayLength;
// use bit_vec::BitBlock;
use bit_array::{BitArray, BitsIn};

type B = u8;
trait Dimension
    where Self: Add<<B as BitsIn>::Output>,
        <Self as Add<<B as BitsIn>::Output>>::Output: Sub<typenum::B1>,
        <<Self as Add<<B as BitsIn>::Output>>::Output as Sub<typenum::B1>>::Output: Div<<B as BitsIn>::Output>,
        <<<Self as Add<<B as BitsIn>::Output>>::Output as Sub<typenum::B1>>::Output as Div<<B as BitsIn>::Output>>::Output: generic_array::ArrayLength<B> {}
type UnitBasisBlade<D: Dimension> = BitArray<B, D>;

struct ScaledBasisBlade<F: Float, D: Dimension> {
    unit_basis_blade: UnitBasisBlade<D>,
    scale: F,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
