#![feature(const_fn)]

extern crate num_traits;
extern crate typenum;
extern crate generic_array;
extern crate alga;
extern crate nalgebra;

pub mod dimension;
pub mod space;
#[macro_use]
pub mod ops;
pub mod unit_basis_blade;
pub mod scaled_basis_blade;
pub mod num;

pub use dimension::*;
pub use space::*;
pub use unit_basis_blade::*;
pub use scaled_basis_blade::*;
pub use ops::*;
pub use num::*;
