use std::ops::Mul;
use dimension::{Dimension, DimensionBitset};
use unit_basis_blade::UnitBasisBlade;
use scaled_basis_blade::ScaledBasisBlade;
use num::Real;

pub trait GeometricProduct<RHS=Self> {
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
