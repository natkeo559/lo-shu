#![allow(incomplete_features)]
#![feature(portable_simd)]
#![feature(generic_const_exprs)]
#![feature(array_try_from_fn)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::many_single_char_names)]

pub mod checkers;
pub mod constructive;
pub mod core;
pub mod group;
pub mod order;
pub mod prelude;

pub use crate::checkers::*;
pub use crate::constructive::*;
pub use crate::core::*;
pub use crate::group::*;
pub use crate::order::*;
pub use crate::prelude::*;