use alga;

pub trait Real: alga::general::Real + Default {}

impl<T> Real for T where T: alga::general::Real + Default {}
