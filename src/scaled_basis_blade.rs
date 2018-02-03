use std::ops::Mul;
use dimension::{CountBits, Dimension};
use ops::GeometricProduct;
use num::Real;
use unit_basis_blade::UnitBasisBlade;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ScaledBasisBlade<R: Real, D: Dimension> {
    scale: R,
    unit_basis_blade: UnitBasisBlade<D>,
}

impl<R: Real, D: Dimension> ScaledBasisBlade<R, D> {
    pub fn zero() -> Self {
        ScaledBasisBlade {
            scale: R::zero(),
            unit_basis_blade: UnitBasisBlade::zero(),
        }
    }

    pub fn new(scale: R, unit_basis_blade: UnitBasisBlade<D>) -> Self {
        if unit_basis_blade.is_zero() || scale.is_zero() {
            Self::zero()
        } else {
            Self {
                scale,
                unit_basis_blade,
            }
        }
    }

    pub fn scale(&self) -> R {
        self.scale
    }

    pub fn unit_basis_blade(&self) -> &UnitBasisBlade<D> {
        &self.unit_basis_blade
    }

    pub fn is_zero(&self) -> bool {
        self.scale.is_zero()
    }
}

impl<R: Real, D: Dimension> From<UnitBasisBlade<D>> for ScaledBasisBlade<R, D> {
    fn from(unit_basis_blade: UnitBasisBlade<D>) -> Self {
        ScaledBasisBlade::new(R::one(), unit_basis_blade)
    }
}

impl<R: Real, D: Dimension> From<(R, UnitBasisBlade<D>)> for ScaledBasisBlade<R, D> {
    fn from(tuple: (R, UnitBasisBlade<D>)) -> Self {
        let (scale, unit_basis_blade) = tuple;

        ScaledBasisBlade::new(scale, unit_basis_blade)
    }
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
            total_swaps += (lbs & rbs).count_bits();
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
