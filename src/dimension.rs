use typenum::{Unsigned, IsLessOrEqual, True};
use generic_array::ArrayLength;

macro_rules! define_max_dimension {
    ($bitset_type:ty, $max_dimension:ident) => {
        use typenum::$max_dimension;
        pub type DimensionBitset = $bitset_type;
        #[allow(dead_code)]
        pub type MaxDimension = $max_dimension;
    }
}

#[cfg(feature = "max-dimension-8")]
define_max_dimension!(u8, U8);
#[cfg(feature = "max-dimension-16")]
define_max_dimension!(u16, U16);
#[cfg(feature = "max-dimension-32")]
define_max_dimension!(u32, U32);
#[cfg(feature = "max-dimension-64")]
define_max_dimension!(u64, U64);

pub trait Dimension: Sized + Default + Unsigned + ArrayLength<bool> + IsLessOrEqual<MaxDimension, Output=True> {}

impl<T> Dimension for T where T: Sized + Default + Unsigned + ArrayLength<bool> + IsLessOrEqual<MaxDimension, Output=True> {}
