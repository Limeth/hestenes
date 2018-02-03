use std::ops::Mul;
use dimension::{Dimension, DimensionBitset};
use basis_blade::{UnitBasisBlade, ScaledBasisBlade};
use num::Real;

trait GeometricProduct<RHS=Self> {
    type Output;

    fn geom(self, rhs: RHS) -> Self::Output;
}

/// Implements an operator on owned types
macro_rules! impl_operator_owned {
    (operator_type: [$($operator_type:tt)+];
     operator_fn: $operator_fn:ident;
     generics: [$($generics:tt)*];
     header: ($lhs:ty, $rhs:ty) -> $output:ty;
     |$lhs_ident:ident, $rhs_ident:ident| $impl:expr) => {
        impl<$($generics)*> $($operator_type)+<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn $operator_fn(self, $rhs_ident: $rhs) -> Self::Output {
                let $lhs_ident = self;
                $impl
            }
        }
    }
}

/// Implements an operator on all owned/borrowed type combinations
macro_rules! impl_operator {
    (operator_type: [$($operator_type:tt)+];
     operator_fn: $operator_fn:ident;
     generics: [$($generics:tt)*];
     header: ($lhs:ty, $rhs:ty) -> $output:ty;
     |&$lhs_ident:ident, &$rhs_ident:ident| $impl:expr) => {
        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            operator_fn: $operator_fn;
            generics: ['a, 'b, $($generics)*];
            header: (&'a $lhs, &'a $rhs) -> $output;
            |$lhs_ident, $rhs_ident| $impl
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            operator_fn: $operator_fn;
            generics: ['b, $($generics)*];
            header: ($lhs, &'b $rhs) -> $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn(&$lhs_ident, $rhs_ident)
            }
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            operator_fn: $operator_fn;
            generics: ['a, $($generics)*];
            header: (&'a $lhs, $rhs) -> $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn($lhs_ident, &$rhs_ident)
            }
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            operator_fn: $operator_fn;
            generics: [$($generics)*];
            header: ($lhs, $rhs) -> $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn(&$lhs_ident, &$rhs_ident)
            }
        }
    }
}

/// Counts the number of bits in a `DimensionBitset`
fn count_bits(mut bitset: DimensionBitset) -> u8 {
    let mut count = 0u8;

    while bitset != 0 {
        count += bitset & 1;
        bitset >>= 1;
    }

    count
}

impl_operator! {
    operator_type: [GeometricProduct];
    operator_fn: geom;
    generics: [R: Real, D: Dimension];
    header: (ScaledBasisBlade<R, D>, ScaledBasisBlade<R, D>) -> ScaledBasisBlade<R, D>;
    |&lhs, &rhs| {
        let mut lbs = lhs.unit_basis_blade().bitset();
        let rbs = rhs.unit_basis_blade().bitset();

        // Check for linear dependency
        if lbs & rbs != 0 {
            // If two blades are linearly dependent, the result is 0.
            return ScaledBasisBlade::zero();
        }

        let mut scale = lhs.scale() * rhs.scale();

        if scale.is_zero() {
            return ScaledBasisBlade::zero();
        }

        let resulting_bitset = lbs | rbs;
        let mut total_swaps = 0;

        while lbs > 1 {
            lbs >>= 1;
            total_swaps += count_bits(lbs & rbs);
        }

        // Negate the scale if the number of swaps was odd
        scale = if total_swaps % 2 == 0 { scale } else { scale.neg() };

        ScaledBasisBlade::new(scale, UnitBasisBlade::new(resulting_bitset))
    }
}

impl_operator! {
    operator_type: [Mul];
    operator_fn: mul;
    generics: [R: Real, D: Dimension];
    header: (ScaledBasisBlade<R, D>, ScaledBasisBlade<R, D>) -> ScaledBasisBlade<R, D>;
    |&lhs, &rhs| {
        GeometricProduct::geom(lhs, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometric_product_1() {
        let a: ScaledBasisBlade<f32, _> = (1.0, ([false, true, true]).into()).into();
        let b: ScaledBasisBlade<f32, _> = (1.0, ([true, false, false]).into()).into();

        assert_eq!(&a * &b, ScaledBasisBlade::new(1.0, [true, true, true].into()));
    }

    #[test]
    fn geometric_product_2() {
        let a: ScaledBasisBlade<f32, _> = (1.0, ([false, true, true]).into()).into();
        let b: ScaledBasisBlade<f32, _> = (1.0, ([true, false, false]).into()).into();

        assert_eq!(&a * &b, ScaledBasisBlade::new(1.0, [true, true, true].into()));
    }
}
