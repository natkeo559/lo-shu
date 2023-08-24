#![allow(incomplete_features)]
#![feature(portable_simd)]
#![feature(generic_const_exprs)]
#![feature(array_try_from_fn)]

pub mod checkers;
pub mod constructive;
pub mod core;
pub mod group;
pub mod order;

pub use crate::checkers::*;
pub use crate::constructive::*;
pub use crate::core::*;
pub use crate::group::*;
pub use crate::order::*;
