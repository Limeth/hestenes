use std::ops::Mul;
use dimension::{Dimension, DimensionBitset};
use unit_basis_blade::UnitBasisBlade;
use scaled_basis_blade::ScaledBasisBlade;
use num::Real;

pub trait GeometricProduct<RHS=Self> {
    type Output;

    fn geom(self, rhs: RHS) -> Self::Output;
}

pub trait OuterProduct<RHS=Self> {
    type Output;

    fn outer(self, rhs: RHS) -> Self::Output;
}

/// Implements a binary operator on owned types
macro_rules! impl_operator_owned {
    (operator_type: [$($operator_type:tt)+];
     inline: [false];
     operator_fn: $operator_fn:ident;
     generics: [$($generics:tt)*];
     header: ($lhs:ty, $rhs:ty) -> $output:ty;
     |$lhs_ident:ident, $rhs_ident:ident| $impl:expr) => {
        impl<$($generics)*> $($operator_type)+<$rhs> for $lhs {
            type Output = $output;

            fn $operator_fn(self, $rhs_ident: $rhs) -> Self::Output {
                let $lhs_ident = self;
                $impl
            }
        }
    };

    (operator_type: [$($operator_type:tt)+];
     inline: [true];
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
     inline: [$($inline:tt)+];
     operator_fn: $operator_fn:ident;
     generics: [$($generics:tt)*];
     header: ($lhs:ty, $rhs:ty) -> $output:ty;
     |&$lhs_ident:ident, &$rhs_ident:ident| $impl:expr) => {
        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            inline: [$($inline)+];
            operator_fn: $operator_fn;
            generics: ['a, 'b, $($generics)*];
            header: (&'a $lhs, &'b $rhs) -> $output;
            |$lhs_ident, $rhs_ident| $impl
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            inline: [$($inline)+];
            operator_fn: $operator_fn;
            generics: ['b, $($generics)*];
            header: ($lhs, &'b $rhs) -> $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn(&$lhs_ident, $rhs_ident)
            }
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            inline: [$($inline)+];
            operator_fn: $operator_fn;
            generics: ['a, $($generics)*];
            header: (&'a $lhs, $rhs) -> $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn($lhs_ident, &$rhs_ident)
            }
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            inline: [$($inline)+];
            operator_fn: $operator_fn;
            generics: [$($generics)*];
            header: ($lhs, $rhs) -> $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn(&$lhs_ident, &$rhs_ident)
            }
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            inline: [$($inline)+];
            operator_fn: $operator_fn;
            generics: ['a, 'b, $($generics)*];
            header: (&'a mut $lhs, &'b mut $rhs) -> $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn(&*$lhs_ident, &*$rhs_ident)
            }
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            inline: [$($inline)+];
            operator_fn: $operator_fn;
            generics: ['b, $($generics)*];
            header: ($lhs, &'b mut $rhs) -> $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn(&$lhs_ident, &*$rhs_ident)
            }
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            inline: [$($inline)+];
            operator_fn: $operator_fn;
            generics: ['a, $($generics)*];
            header: (&'a mut $lhs, $rhs) -> $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn(&*$lhs_ident, &$rhs_ident)
            }
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            inline: [$($inline)+];
            operator_fn: $operator_fn;
            generics: ['a, 'b, $($generics)*];
            header: (&'a mut $lhs, &'b $rhs) -> $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn(&*$lhs_ident, $rhs_ident)
            }
        }

        impl_operator_owned! {
            operator_type: [$($operator_type)+];
            inline: [$($inline)+];
            operator_fn: $operator_fn;
            generics: ['a, 'b, $($generics)*];
            header: (&'a $lhs, &'b mut $rhs) -> $output;
            |$lhs_ident, $rhs_ident| {
                $($operator_type)+::$operator_fn($lhs_ident, &*$rhs_ident)
            }
        }
    }
}

/// Implements an unary operator on owned types
macro_rules! impl_unary_operator_owned {
    (operator_type: [$($operator_type:tt)+];
     inline: [false];
     operator_fn: $operator_fn:ident;
     generics: [$($generics:tt)*];
     header: ($input:ty) -> $output:ty;
     |$self:ident| $impl:expr) => {
        impl<$($generics)*> $($operator_type)+ for $input {
            type Output = $output;

            fn $operator_fn(self) -> Self::Output {
                let $self = self;
                $impl
            }
        }
    };

    (operator_type: [$($operator_type:tt)+];
     inline: [true];
     operator_fn: $operator_fn:ident;
     generics: [$($generics:tt)*];
     header: ($input:ty) -> $output:ty;
     |$self:ident| $impl:expr) => {
        impl<$($generics)*> $($operator_type)+ for $input {
            type Output = $output;

            #[inline]
            fn $operator_fn(self) -> Self::Output {
                let $self = self;
                $impl
            }
        }
    }
}

macro_rules! impl_unary_operator {
    (operator_type: [$($operator_type:tt)+];
     inline: [$($inline:tt)+];
     operator_fn: $operator_fn:ident;
     generics: [$($generics:tt)*];
     header: ($input:ty) -> $output:ty;
     |&$self:ident| $impl:expr) => {
        impl_unary_operator_owned! {
            operator_type: [$($operator_type)+];
            inline: [$($inline)+];
            operator_fn: $operator_fn;
            generics: ['a, $($generics)*];
            header: (&'a $input) -> $output;
            |$self| $impl
        }

        impl_unary_operator_owned! {
            operator_type: [$($operator_type)+];
            inline: [$($inline)+];
            operator_fn: $operator_fn;
            generics: ['a, $($generics)*];
            header: (&'a mut $input) -> $output;
            |$self| {
                $($operator_type)+::$operator_fn(&*$self)
            }
        }

        impl_unary_operator_owned! {
            operator_type: [$($operator_type)+];
            inline: [$($inline)+];
            operator_fn: $operator_fn;
            generics: [$($generics)*];
            header: ($input) -> $output;
            |$self| {
                $($operator_type)+::$operator_fn(&$self)
            }
        }
    }
}

macro_rules! impl_operator_outer {
    (inline: [$($inline:tt)+];
     generics: [$($generics:tt)*];
     header: ($lhs:ty, $rhs:ty) -> $output:ty;
     |&$lhs_ident:ident, &$rhs_ident:ident| $impl:expr) => {
        impl_operator! {
            operator_type: [OuterProduct];
            inline: [$($inline)+];
            operator_fn: outer;
            generics: [$($generics)*];
            header: ($lhs, $rhs) -> $output;
            |&$lhs_ident, &$rhs_ident| $impl
        }

        impl_operator! {
            operator_type: [BitXor];
            inline: [true];
            operator_fn: bitxor;
            generics: [$($generics)*];
            header: ($lhs, $rhs) -> $output;
            |&lhs, &rhs| {
                OuterProduct::outer(lhs, rhs)
            }
        }
    }
}
