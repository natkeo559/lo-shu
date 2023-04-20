use crate::order::Params;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrderThree;

impl Params for OrderThree {
    const ORDER: usize = 3;
    const ELEMENTS: usize = Self::ORDER * Self::ORDER;
    const MODULUS: usize = 10;
    const MAGIC_SUM: u32 = 15;
    const PERMUTATIONS: usize = 362880;
    const CONSTRAINT_VECTORS: usize = Self::ORDER * 2 + 2;
    const ELEMENT_BITS: usize = 8;
}
