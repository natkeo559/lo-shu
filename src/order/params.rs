/// A trait for parameter sets defining the properties of a square.
pub trait Params {
    const ORDER: usize;
    const ELEMENTS: usize;
    const MAGIC_SUM: u32;
    const CONSTRAINT_VECTORS: usize;
}

/// Macro to generate implementations of the Params trait for different parameter sets.
macro_rules! impl_parameter_set {
    ($order:literal, $name:tt) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name;

        impl Params for $name {
            const ORDER: usize = $order;
            const ELEMENTS: usize = Self::ORDER * Self::ORDER;
            const MAGIC_SUM: u32 =
                (((Self::ELEMENTS * (Self::ELEMENTS + 1)) / 2) / Self::ORDER) as u32;
            const CONSTRAINT_VECTORS: usize = Self::ORDER * 2 + 2;
        }
    };
}

// Generate implementations for specific parameter sets.
impl_parameter_set!(3, O3);
impl_parameter_set!(4, O4);
impl_parameter_set!(5, O5);
impl_parameter_set!(25, O25);
impl_parameter_set!(301, O301);
impl_parameter_set!(301, O1001);
