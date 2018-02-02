use std::ops::Mul;
use dimension::Dimension;
use basis_blade::ScaledBasisBlade;
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
     header: ($lhs:ty, $rhs:ty) => $output:ty;
     |$lhs_ident:ident, $rhs_ident:ident| $impl:expr) => {
        impl<$($generics)*> $($operator_type)+<$rhs> for $lhs {
            type Output = $output;

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
     header: ($lhs:ty, $rhs:ty) => $output:ty;
     |&$lhs_ident:ident, &$rhs_ident:ident| $impl:expr) => {
        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            operator_fn: $operator_fn;
            generics: ['a, 'b, $($generics)*];
            header: (&'a $lhs, &'a $rhs) => $output;
            |$lhs_ident, $rhs_ident| $impl
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            operator_fn: $operator_fn;
            generics: ['b, $($generics)*];
            header: ($lhs, &'b $rhs) => $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn(&$lhs_ident, $rhs_ident)
            }
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            operator_fn: $operator_fn;
            generics: ['a, $($generics)*];
            header: (&'a $lhs, $rhs) => $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn($lhs_ident, &$rhs_ident)
            }
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            operator_fn: $operator_fn;
            generics: [$($generics)*];
            header: ($lhs, $rhs) => $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn(&$lhs_ident, &$rhs_ident)
            }
        }
    }
}

impl_operator! {
    operator_type: [GeometricProduct];
    operator_fn: geom;
    generics: [R: Real, D: Dimension];
    header: (ScaledBasisBlade<R, D>, ScaledBasisBlade<R, D>) => ScaledBasisBlade<R, D>;
    |&_lhs, &_rhs| {
        Default::default()
    }
}

impl_operator! {
    operator_type: [Mul];
    operator_fn: mul;
    generics: [R: Real, D: Dimension];
    header: (ScaledBasisBlade<R, D>, ScaledBasisBlade<R, D>) => ScaledBasisBlade<R, D>;
    |&lhs, &rhs| {
        GeometricProduct::geom(lhs, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use basis_blade::UnitBasisBlade;

    #[test]
    fn geometric_product() {
        let a: ScaledBasisBlade<f32, _> = UnitBasisBlade::<_>::from([false, true, true]).into();
        let b: ScaledBasisBlade<f32, _> = UnitBasisBlade::<_>::from([true, false, false]).into();

        // TODO
        assert_eq!(&a * &b, ScaledBasisBlade::new([false, false, false].into(), 0.0));
    }
}
