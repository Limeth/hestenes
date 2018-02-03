use std::ops::{BitAnd, BitOr, BitXor};
use std::marker::PhantomData;
use generic_array::GenericArray;
use dimension::{CountBits, Dimension, DimensionBitset, BitsetMask, Grade};

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
    pub fn zero() -> Self {
        UnitBasisBlade {
            bitset: 0,
            _marker: PhantomData,
        }
    }

    pub fn new(bitset: DimensionBitset) -> Self {
        UnitBasisBlade {
            bitset,
            _marker: PhantomData,
        }
    }

    /// Indices start from 0
    #[inline]
    pub fn contains_basis_vector(&self, index: u8) -> bool {
        ((1 << index) & self.bitset) != 0
    }

    pub fn basis_vectors(&self) -> GenericArray<bool, D> {
        GenericArray::generate(|index| self.contains_basis_vector(index as u8))
    }

    pub fn bitset(&self) -> DimensionBitset {
        self.bitset
    }

    pub fn is_zero(&self) -> bool {
        self.bitset == 0
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

impl<D: Dimension> Grade for UnitBasisBlade<D> {
    fn grade(&self) -> u8 {
        self.bitset.count_bits()
    }
}

impl_operator! {
    operator_type: [BitAnd];
    operator_fn: bitand;
    generics: [D: Dimension];
    header: (UnitBasisBlade<D>, UnitBasisBlade<D>) -> UnitBasisBlade<D>;
    |&lhs, &rhs| {
        UnitBasisBlade::new(BitAnd::bitand(lhs.bitset, rhs.bitset))
    }
}

impl_operator! {
    operator_type: [BitOr];
    operator_fn: bitor;
    generics: [D: Dimension];
    header: (UnitBasisBlade<D>, UnitBasisBlade<D>) -> UnitBasisBlade<D>;
    |&lhs, &rhs| {
        UnitBasisBlade::new(BitOr::bitor(lhs.bitset, rhs.bitset))
    }
}

impl_operator! {
    operator_type: [BitXor];
    operator_fn: bitxor;
    generics: [D: Dimension];
    header: (UnitBasisBlade<D>, UnitBasisBlade<D>) -> UnitBasisBlade<D>;
    |&lhs, &rhs| {
        UnitBasisBlade::new(BitXor::bitxor(lhs.bitset, rhs.bitset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::U3;

    #[test]
    fn unit_basis_blade_from_bool_array() {
        let blade: UnitBasisBlade<_> = [false, true, true].into();

        assert_eq!(blade.bitset, 6);
        assert_eq!(blade.basis_vectors(), [false, true, true].into());
    }

    #[test]
    fn unit_basis_blade_from_u8() {
        let blade = UnitBasisBlade::<U3>::from_u8(255u8);

        assert_eq!(blade.bitset, 7);
        assert_eq!(blade.basis_vectors(), [true, true, true].into());
    }
}
