use crate::Square;

macro_rules! impl_int_perms {
    ($t: ty) => {
        impl Square<$t> {
            ///
            /// Based on:
            /// https://stackoverflow.com/questions/31216097/given-n-and-k-return-the-kth-permutation-sequence
            /// 
            pub fn kth_perm(k: i32) -> Square<$t> {
                let mut n = [1, 2, 3, 4, 5, 6, 7, 8, 9];
                let mut indeces = [0; 9];

                let mut divisor = 1;
                for place in 1..10 {
                    if k / divisor == 0 {
                        break;
                    }
                    indeces[9 - place] = (k / divisor) % place as i32;
                    divisor *= place as i32;
                }
                for i in 0..9 {
                    let index = indeces[i] as usize + i;
                    if index != i {
                        let temp = n[index];
                        let mut j = index;
                        while j > i {
                            n[j] = n[j - 1];
                            j -= 1;
                        }
                        n[i] = temp;
                    }
                }
                Square { array: n }
            }
        }
    };
}

macro_rules! impl_float_perms {
    ($t: ty) => {
        impl Square<$t> {
            ///
            /// Based on:
            /// https://stackoverflow.com/questions/31216097/given-n-and-k-return-the-kth-permutation-sequence
            /// 
            pub fn kth_perm(k: i32) -> Square<$t> {
                let mut n = [1., 2., 3., 4., 5., 6., 7., 8., 9.];
                let mut indeces = [0; 9];

                let mut divisor = 1;
                for place in 1..10 {
                    if k / divisor == 0 {
                        break;
                    }
                    indeces[9 - place] = (k / divisor) % place as i32;
                    divisor *= place as i32;
                }
                for i in 0..9 {
                    let index = indeces[i] as usize + i;
                    if index != i {
                        let temp = n[index];
                        let mut j = index;
                        while j > i {
                            n[j] = n[j - 1];
                            j -= 1;
                        }
                        n[i] = temp;
                    }
                }
                Square { array: n }
            }
        }
    };
}

impl_int_perms!(u8);
impl_int_perms!(u16);
impl_float_perms!(f32);
impl_float_perms!(f64);
