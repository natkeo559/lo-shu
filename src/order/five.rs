use crate::order::Params;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrderFive;

impl Params for OrderFive {
    const ORDER: usize = 5;
    const ELEMENTS: usize = Self::ORDER * Self::ORDER;
    const MODULUS: usize = 10;
    const MAGIC_SUM: u32 = 65;
    const PERMUTATIONS: usize = 0;
    const CONSTRAINT_VECTORS: usize = Self::ORDER * 2 + 2;
    const ELEMENT_BITS: usize = 16;
}
