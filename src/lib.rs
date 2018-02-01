extern crate num_traits;
extern crate typenum;
extern crate generic_array;

use std::marker::PhantomData;
use num_traits::Float;
use typenum::Unsigned;
use generic_array::{ArrayLength, GenericArray};

// {{{ Dimension
pub trait Dimension: Sized + Unsigned + ArrayLength<bool> {}

impl<T> Dimension for T where T: Sized + Unsigned + ArrayLength<bool> {}
// }}} Dimension

// {{{ UnitBasisBlade
#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct UnitBasisBlade<D: Dimension> {
    bitset: usize,
    _marker: PhantomData<D>,
}

impl<D: Dimension> UnitBasisBlade<D> {
    /// Indices start from 0
    pub fn contains_basis_vector(&self, index: u8) -> bool {
        ((1 << index) & self.bitset) != 0
    }
}

impl<D: Dimension, T: Into<GenericArray<bool, D>>> From<T> for UnitBasisBlade<D> {
    fn from(other: T) -> Self {
        UnitBasisBlade {
            bitset: other.into().iter().enumerate().fold(0, |folded, (index, item)| {
                folded + if *item { 1 << index } else { 0 } 
            }),
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_unit_basis_blade_from {
    ($($type:ty),+) => {
        $(
            impl<D: Dimension> From<$type> for UnitBasisBlade<D> {
                fn from(other: $type) -> Self {
                    UnitBasisBlade {
                        bitset: other as usize,
                        _marker: PhantomData,
                    }
                }
            }
        )+
    }
}

impl_unit_basis_blade_from!(usize, u8, u16, u32, u64);
// }}} UnitBasisBlade

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct ScaledBasisBlade<F: Float, D: Dimension> {
    unit_basis_blade: UnitBasisBlade<D>,
    scale: F,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let blade: UnitBasisBlade<typenum::U3> = 6u32.into();
        let blade: UnitBasisBlade<typenum::U3> = [false, true, true].into();
        panic!("{:?}", blade);
        assert_eq!(2 + 2, 4);
    }
}
