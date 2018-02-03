#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

extern crate num_traits;
extern crate typenum;
extern crate generic_array;
extern crate alga;

pub mod dimension;
#[macro_use]
pub mod ops;
pub mod basis_blade;
pub mod num;

pub use dimension::*;
pub use basis_blade::*;
pub use ops::*;
pub use num::*;
