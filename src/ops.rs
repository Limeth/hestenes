use std::ops::Mul;
use dimension::Dimension;
use basis_blade::ScaledBasisBlade;
use num::Real;

impl<'a, R: Real, D: Dimension> Mul for &'a ScaledBasisBlade<R, D> {
    type Output = ScaledBasisBlade<R, D>;

    fn mul(self, rhs: Self) -> Self::Output {
        // TODO
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use basis_blade::UnitBasisBlade;

    #[test]
    fn it_works() {
        let a: ScaledBasisBlade<f32, _> = UnitBasisBlade::<_>::from([false, true, true]).into();
        let b: ScaledBasisBlade<f32, _> = UnitBasisBlade::<_>::from([true, false, false]).into();

        // TODO
        assert_eq!(a.mul(&b), ScaledBasisBlade::new([false, false, false].into(), 0.0));
    }
}
