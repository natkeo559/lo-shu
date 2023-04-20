use crate::order::Params;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrderFour;

impl Params for OrderFour {
    const ORDER: usize = 4;
    const ELEMENTS: usize = Self::ORDER * Self::ORDER;
    const MODULUS: usize = 10;
    const MAGIC_SUM: u32 = 34;
    const PERMUTATIONS: usize = 20922789888000;
    const CONSTRAINT_VECTORS: usize = Self::ORDER * 2 + 2;
    const ELEMENT_BITS: usize = 16;
}
