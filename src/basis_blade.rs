use std::marker::PhantomData;
use generic_array::GenericArray;
use dimension::{Dimension, DimensionBitset, BitsetMask};
use num::Real;

// {{{ UnitBasisBlade
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct UnitBasisBlade<D: Dimension> {
    bitset: DimensionBitset,
    _marker: PhantomData<D>,
}

macro_rules! impl_unit_basis_blade_from {
    ($name:ident, $type:ty) => {
        pub fn $name(other: $type) -> Self {
            Self {
                bitset: (other as DimensionBitset) & <D as BitsetMask<DimensionBitset>>::bitset_mask(),
                _marker: PhantomData,
            }
        }
    }
}

impl<D: Dimension> UnitBasisBlade<D> {
    /// Indices start from 0
    #[inline]
    pub fn contains_basis_vector(&self, index: u8) -> bool {
        ((1 << index) & self.bitset) != 0
    }

    impl_unit_basis_blade_from!(from_usize, usize);
    impl_unit_basis_blade_from!(from_u8, u8);
    impl_unit_basis_blade_from!(from_u16, u16);
    impl_unit_basis_blade_from!(from_u32, u32);
    impl_unit_basis_blade_from!(from_u64, u64);
}

impl<D: Dimension, T: Into<GenericArray<bool, D>>> From<T> for UnitBasisBlade<D> {
    fn from(other: T) -> Self {
        UnitBasisBlade {
            bitset: other.into().iter().enumerate().fold(0, |folded, (index, item)| {
                folded | if *item { 1 << index } else { 0 }
            }),
            _marker: PhantomData,
        }
    }
}
// }}} UnitBasisBlade

// {{{ ScaledBasisBlade
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ScaledBasisBlade<R: Real, D: Dimension> {
    unit_basis_blade: UnitBasisBlade<D>,
    scale: R,
}

impl<R: Real, D: Dimension> ScaledBasisBlade<R, D> {
    pub fn new(unit_basis_blade: UnitBasisBlade<D>, scale: R) -> Self {
        Self {
            unit_basis_blade,
            scale,
        }
    }
}

impl<R: Real, D: Dimension> From<UnitBasisBlade<D>> for ScaledBasisBlade<R, D> {
    fn from(unit_basis_blade: UnitBasisBlade<D>) -> Self {
        ScaledBasisBlade {
            unit_basis_blade,
            scale: R::one(),
        }
    }
}
// }}} ScaledBasisBlade

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::U3;

    #[test]
    fn unit_basis_blade_from_u8() {
        let blade = UnitBasisBlade::<U3>::from_u8(255u8);

        assert_eq!(blade.bitset, 7);
    }
}
