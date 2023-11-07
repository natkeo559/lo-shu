#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{IndexConst, MessageSolver, O3};

#[inline]
pub fn from_builder() -> Result<(), anyhow::Error> {
    MessageSolver::<O3>::default_build()
        .threads(16)
        .upper_bound(O3::MAX_INDEX)
        .n(2)
        .echo(true)
        .output_dir("examples/collected/orderfour/")
        .filename("SolverMPSC")
        .execute()?;

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    from_builder()
}
