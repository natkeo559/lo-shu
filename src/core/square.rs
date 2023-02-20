use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square3<T: Clone + Copy> {
    pub array: [T; 9],
}

macro_rules! impl_int_square3 {
    ($t: ty) => {
        impl Square3<$t> {
            pub fn first() -> Square3<$t> {
                Square3 {
                    array: [1, 2, 3, 4, 5, 6, 7, 8, 9],
                }
            }

            pub fn zeros() -> Square3<$t> {
                Square3 { array: [0; 9] }
            }

            pub fn fill(n: $t) -> Square3<$t> {
                Square3 { array: [n; 9] }
            }
        }
    };
}

macro_rules! impl_float_square3 {
    ($t: ty) => {
        impl Square3<$t> {
            pub fn first() -> Square3<$t> {
                Square3 {
                    array: [1., 2., 3., 4., 5., 6., 7., 8., 9.],
                }
            }

            pub fn zeros() -> Square3<$t> {
                Square3 { array: [0.; 9] }
            }

            pub fn fill(n: $t) -> Square3<$t> {
                Square3 { array: [n; 9] }
            }
        }
    };
}

macro_rules! impl_display_square3 {
    ($t: ty) => {
        impl fmt::Display for Square3<$t> {
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

impl<T: Copy + Clone> Square3<T> {
    pub fn from_slice(slice: &[T; 9]) -> Square3<T> {
        Square3 { array: *slice }
    }
}

impl_int_square3!(u8);
impl_int_square3!(u16);
impl_float_square3!(f32);
impl_float_square3!(f64);

impl_display_square3!(u8);
impl_display_square3!(u16);
impl_display_square3!(f32);
impl_display_square3!(f64);

#[cfg(test)]
mod test_square3 {
    use crate::Square3;

    #[test]
    fn test_first() {
        let a = Square3::<u8>::first();

        assert_eq!(
            a,
            Square3 {
                array: [1, 2, 3, 4, 5, 6, 7, 8, 9],
            }
        );

        let a = Square3::<u16>::first();

        assert_eq!(
            a,
            Square3 {
                array: [1, 2, 3, 4, 5, 6, 7, 8, 9],
            }
        );

        let a = Square3::<f32>::first();

        assert_eq!(
            a,
            Square3 {
                array: [1., 2., 3., 4., 5., 6., 7., 8., 9.],
            }
        );

        let a = Square3::<f64>::first();

        assert_eq!(
            a,
            Square3 {
                array: [1., 2., 3., 4., 5., 6., 7., 8., 9.],
            }
        );
    }

    #[test]
    fn test_zeros() {
        let a = Square3::<u8>::zeros();

        assert_eq!(a, Square3 { array: [0; 9] });

        let a = Square3::<u16>::zeros();

        assert_eq!(a, Square3 { array: [0; 9] });

        let a = Square3::<f32>::zeros();

        assert_eq!(a, Square3 { array: [0.; 9] });

        let a = Square3::<f64>::zeros();

        assert_eq!(a, Square3 { array: [0.; 9] });
    }

    #[test]
    fn test_fill() {
        for i in 1u8..10 {
            let a = Square3::<u8>::fill(i);
            assert_eq!(a, Square3 { array: [i; 9] });
        }

        for i in 1u16..10 {
            let a = Square3::<u16>::fill(i);
            assert_eq!(a, Square3 { array: [i; 9] });
        }

        for i in 1..10 {
            let i = i as f32;
            let a = Square3::<f32>::fill(i);
            assert_eq!(a, Square3 { array: [i; 9] });
        }

        for i in 1..10 {
            let i = i as f64;
            let a = Square3::<f64>::fill(i);
            assert_eq!(a, Square3 { array: [i; 9] });
        }
    }
}
