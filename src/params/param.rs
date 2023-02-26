pub trait Params {
    const ORDER: usize;
    const ELEMENTS: usize;
    const MODULUS: usize;
    const MAGIC_SUM: u32;
    const PERMUTATIONS: usize;
    const CONSTRAINT_VECTORS: usize;
}
