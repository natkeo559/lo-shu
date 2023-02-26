#![allow(incomplete_features)]
#![feature(portable_simd)]
#![feature(generic_const_exprs)]

pub mod core;
pub mod params;

pub use crate::core::*;
pub use crate::params::*;
