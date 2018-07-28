use std::ops::{BitAnd, BitOr, BitXor};
use std::marker::PhantomData;
use generic_array::GenericArray;
use dimension::{CountBits, Dimension, DimensionBitset, BitsetMask, Grade};
use typenum::*;

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

macro_rules! impl_unit_basis_blade_from_integer {
    ($type:ty) => {
        impl<D: Dimension> From<$type> for UnitBasisBlade<D> {
            fn from(other: $type) -> Self {
                Self {
                    bitset: (other as DimensionBitset) & <D as BitsetMask<DimensionBitset>>::bitset_mask(),
                    _marker: PhantomData,
                }
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
}

#[cfg(any(feature = "max-dimension-8", feature = "max-dimension-16", feature = "max-dimension-32", feature = "max-dimension-64"))]
impl_unit_basis_blade_from_integer!(u8);

#[cfg(any(feature = "max-dimension-16", feature = "max-dimension-32", feature = "max-dimension-64"))]
impl_unit_basis_blade_from_integer!(u16);

#[cfg(any(feature = "max-dimension-32", feature = "max-dimension-64"))]
impl_unit_basis_blade_from_integer!(u32);

#[cfg(feature = "max-dimension-64")]
impl_unit_basis_blade_from_integer!(u64);

impl<D: Dimension> From<GenericArray<bool, D>> for UnitBasisBlade<D> {
    fn from(other: GenericArray<bool, D>) -> Self {
        UnitBasisBlade {
            bitset: other.iter().enumerate().fold(0, |folded, (index, item)| {
                folded | if *item { 1 << index } else { 0 }
            }),
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_unit_basis_blade_from_array {
    ($dimension_ty:ty, $dimension_expr:expr) => {
        impl From<[bool; $dimension_expr]> for UnitBasisBlade<$dimension_ty> {
            fn from(other: [bool; $dimension_expr]) -> Self {
                GenericArray::<bool, $dimension_ty>::from(other).into()
            }
        }
    }
}

macro_rules! impl_unit_basis_blade_from_array_multiple {
    ($($dimension_ty:ty, $dimension_expr:expr);+) => {
        $(
            impl_unit_basis_blade_from_array!($dimension_ty, $dimension_expr);
        )+
    }
}

#[cfg(any(feature = "max-dimension-8", feature = "max-dimension-16", feature = "max-dimension-32", feature = "max-dimension-64"))]
impl_unit_basis_blade_from_array_multiple!(U1, 1; U2, 2; U3, 3; U4, 4; U5, 5; U6, 6; U7, 7; U8, 8);

#[cfg(any(feature = "max-dimension-16", feature = "max-dimension-32", feature = "max-dimension-64"))]
impl_unit_basis_blade_from_array_multiple!(U9, 9; U10, 10; U11, 11; U12, 12; U13, 13; U14, 14; U15, 15; U16, 16);

#[cfg(any(feature = "max-dimension-32", feature = "max-dimension-64"))]
impl_unit_basis_blade_from_array_multiple!(U17, 17; U18, 18; U19, 19; U20, 20; U21, 21; U22, 22; U23, 23; U24, 24; U25, 25; U26, 26; U27, 27; U28, 28; U29, 29; U30, 30; U31, 31; U32, 32);

#[cfg(feature = "max-dimension-64")]
impl_unit_basis_blade_from_array_multiple!(U33, 33; U34, 34; U35, 35; U36, 36; U37, 37; U38, 38; U39, 39; U40, 40; U41, 41; U42, 42; U43, 43; U44, 44; U45, 45; U46, 46; U47, 47; U48, 48; U49, 49; U50, 50; U51, 51; U52, 52; U53, 53; U54, 54; U55, 55; U56, 56; U57, 57; U58, 58; U59, 59; U60, 60; U61, 61; U62, 62; U63, 63; U64, 64);

impl<D: Dimension> Grade for UnitBasisBlade<D> {
    fn grade(&self) -> u8 {
        self.bitset.count_bits()
    }
}

impl_operator! {
    operator_type: [BitAnd];
    inline: [true];
    operator_fn: bitand;
    generics: [D: Dimension];
    header: (UnitBasisBlade<D>, UnitBasisBlade<D>) -> UnitBasisBlade<D>;
    |&lhs, &rhs| {
        UnitBasisBlade::new(BitAnd::bitand(lhs.bitset, rhs.bitset))
    }
}

impl_operator! {
    operator_type: [BitOr];
    inline: [true];
    operator_fn: bitor;
    generics: [D: Dimension];
    header: (UnitBasisBlade<D>, UnitBasisBlade<D>) -> UnitBasisBlade<D>;
    |&lhs, &rhs| {
        UnitBasisBlade::new(BitOr::bitor(lhs.bitset, rhs.bitset))
    }
}

impl_operator! {
    operator_type: [BitXor];
    inline: [true];
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
        let blade = UnitBasisBlade::<U3>::from(0xFF);

        // Check that a bitmap was applied
        assert_eq!(blade.bitset, 7);
        assert_eq!(blade.basis_vectors(), [true, true, true].into());
    }
}
