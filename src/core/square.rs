use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Square<T: Clone + Copy> {
    pub array: [T; 9],
}

macro_rules! impl_int_square {
    ($t: ty) => {
        impl Square<$t> {
            pub fn first() -> Square<$t> {
                Square {
                    array: [1, 2, 3, 4, 5, 6, 7, 8, 9],
                }
            }

            pub fn zeros() -> Square<$t> {
                Square { array: [0; 9] }
            }

            pub fn fill(n: $t) -> Square<$t> {
                Square { array: [n; 9] }
            }
        }
    };
}

macro_rules! impl_float_square {
    ($t: ty) => {
        impl Square<$t> {
            pub fn first() -> Square<$t> {
                Square {
                    array: [1., 2., 3., 4., 5., 6., 7., 8., 9.],
                }
            }

            pub fn zeros() -> Square<$t> {
                Square { array: [0.; 9] }
            }

            pub fn fill(n: $t) -> Square<$t> {
                Square { array: [n; 9] }
            }
        }
    };
}

macro_rules! impl_display_square {
    ($t: ty) => {
        impl fmt::Display for Square<$t> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "[{} {} {}\n {} {} {}\n {} {} {}]",
                    self.array.get(0).unwrap(),
                    self.array.get(1).unwrap(),
                    self.array.get(2).unwrap(),
                    self.array.get(3).unwrap(),
                    self.array.get(4).unwrap(),
                    self.array.get(5).unwrap(),
                    self.array.get(6).unwrap(),
                    self.array.get(7).unwrap(),
                    self.array.get(8).unwrap()
                )
            }
        }
    };
}

impl<T: Copy + Clone> Square<T> {
    pub fn from_slice(slice: &[T; 9]) -> Square<T> {
        Square { array: *slice }
    }
}

impl_int_square!(u8);
impl_int_square!(u16);
impl_float_square!(f32);
impl_float_square!(f64);

impl_display_square!(u8);
impl_display_square!(u16);
impl_display_square!(f32);
impl_display_square!(f64);

#[cfg(test)]
mod mod_tests {
    #[test]
    fn test_try() {}
}
