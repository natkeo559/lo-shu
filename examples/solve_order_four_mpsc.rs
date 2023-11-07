#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{IndexConst, MessageSolver, O4};

#[inline]
pub fn from_builder() -> Result<(), anyhow::Error> {
    MessageSolver::<O4>::default_build()
        .threads(16)
        .upper_bound(O4::MAX_INDEX)
        .n(40)
        .echo(true)
        .start(80867885530)
        .generate_d(false)
        .output_dir("examples/collected/orderfour/")
        .filename("SolverMPSC")
        .execute()?;

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    from_builder()
}
