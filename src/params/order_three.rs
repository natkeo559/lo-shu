use crate::params::Params;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrderThree;

impl Params for OrderThree {
    const ORDER: usize = 3;
    const ELEMENTS: usize = 9;
    const MODULUS: usize = 10;
    const MAGIC_SUM: u32 = 15;
    const PERMUTATIONS: usize = 362880;
    const CONSTRAINT_VECTORS: usize = 8;
}
