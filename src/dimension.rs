use typenum::{Unsigned, IsLessOrEqual, True};
use generic_array::ArrayLength;

// TODO: Rename to Grade
pub trait BitsetMask<T> {
    fn bitset_mask() -> T;
}

macro_rules! define_max_dimension {
    ($bitset_type:ty, $max_dimension:ident, $to_uint:ident) => {
        use typenum::$max_dimension;
        pub type DimensionBitset = $bitset_type;
        #[allow(dead_code)]
        pub type MaxDimension = $max_dimension;

        impl<T: Dimension> BitsetMask<DimensionBitset> for T {
            #[inline]
            fn bitset_mask() -> DimensionBitset {
                ((1 as $bitset_type) << T::$to_uint()) - 1
            }
        }
    }
}

#[cfg(feature = "max-dimension-8")]
define_max_dimension!(u8, U8, to_u8);
#[cfg(feature = "max-dimension-16")]
define_max_dimension!(u16, U16, to_u16);
#[cfg(feature = "max-dimension-32")]
define_max_dimension!(u32, U32, to_u32);
#[cfg(feature = "max-dimension-64")]
define_max_dimension!(u64, U64, to_u64);

pub trait Dimension: Sized + Default + Unsigned + ArrayLength<bool> + IsLessOrEqual<MaxDimension, Output=True> {}

impl<T> Dimension for T where T: Sized + Default + Unsigned + ArrayLength<bool> + IsLessOrEqual<MaxDimension, Output=True> {}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::U6;

    #[test]
    fn unit_basis_blade_from_u8() {
        let mask = U6::bitset_mask();

        assert_eq!(mask, 63);
    }
}
