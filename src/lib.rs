#![allow(incomplete_features)]
#![feature(portable_simd)]
#![feature(generic_const_exprs)]

pub mod checkers;
pub mod core;
pub mod group;
pub mod order;

pub use crate::checkers::*;
pub use crate::core::*;
pub use crate::group::*;
pub use crate::order::*;
