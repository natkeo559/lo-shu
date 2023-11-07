#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{IndexConst, MessageSolver, O5};

#[inline]
pub fn from_builder() -> Result<(), anyhow::Error> {
    MessageSolver::<O5>::default_build()
        .threads(16)
        .upper_bound(O5::MAX_INDEX)
        .n(40)
        .echo(true)
        .start(100000000000000000000000)
        .generate_d(false)
        .output_dir("examples/collected/orderfive/")
        .filename("SolverMPSC")
        .execute()?;

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    // message_solver(16);
    from_builder()
}
