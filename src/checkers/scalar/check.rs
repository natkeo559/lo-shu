use crate::{OrderFour, OrderThree, Params, Permutation};

impl Permutation<OrderThree> {
    /// Checks if a Permutation of element type T, order N is magic.
    /// Computations are scalar.
    ///
    /// # Safety
    ///
    /// Use of `get_unchecked` is unsafe. For a safe abstraction, use `get` to return references to Square elements.
    pub unsafe fn check_s_unsafe(&self) -> Option<Self> {
        let a: u8 = *self.square.get_unchecked(0);
        let b: u8 = *self.square.get_unchecked(1);
        let c: u8 = *self.square.get_unchecked(2);
        let d: u8 = *self.square.get_unchecked(3);
        let e: u8 = *self.square.get_unchecked(4);
        let f: u8 = *self.square.get_unchecked(5);
        let g: u8 = *self.square.get_unchecked(6);
        let h: u8 = *self.square.get_unchecked(7);
        let i: u8 = *self.square.get_unchecked(8);

        let s1 = a + b + c;
        let s2 = a + e + i;
        let s3 = a + d + g;
        let s4 = b + e + h;
        let s5 = c + f + i;
        let s6 = c + e + g;
        let s7 = d + e + f;
        let s8 = g + h + i;

        match [s1, s2, s3, s4, s5, s6, s7, s8]
            .into_iter()
            .all(|sum| sum == OrderThree::MAGIC_SUM as u8)
        {
            true => Some(*self),
            false => None,
        }
    }

    /// Checks if a Permutation of element type T, order N is magic.
    /// Computations are scalar.
    pub fn check_s(&self) -> Option<Self> {
        let a: u8 = *self.square.get(0).unwrap();
        let b: u8 = *self.square.get(1).unwrap();
        let c: u8 = *self.square.get(2).unwrap();
        let d: u8 = *self.square.get(3).unwrap();
        let e: u8 = *self.square.get(4).unwrap();
        let f: u8 = *self.square.get(5).unwrap();
        let g: u8 = *self.square.get(6).unwrap();
        let h: u8 = *self.square.get(7).unwrap();
        let i: u8 = *self.square.get(8).unwrap();

        let s1 = a + b + c;
        let s2 = a + e + i;
        let s3 = a + d + g;
        let s4 = b + e + h;
        let s5 = c + f + i;
        let s6 = c + e + g;
        let s7 = d + e + f;
        let s8 = g + h + i;

        match [s1, s2, s3, s4, s5, s6, s7, s8]
            .into_iter()
            .all(|sum| sum == OrderThree::MAGIC_SUM as u8)
        {
            true => Some(*self),
            false => None,
        }
    }
}

impl Permutation<OrderFour> {
    /// Checks if a Permutation of element type T, order N is magic.
    /// Computations are scalar.
    ///
    /// # Safety
    ///
    /// Use of `get_unchecked` is unsafe. For a safe abstraction, use `get` to return references to Square elements.
    pub unsafe fn check_s_unsafe(&self) -> Option<Self> {
        let a: u8 = *self.square.get_unchecked(0);
        let b: u8 = *self.square.get_unchecked(1);
        let c: u8 = *self.square.get_unchecked(2);
        let d: u8 = *self.square.get_unchecked(3);
        let e: u8 = *self.square.get_unchecked(4);
        let f: u8 = *self.square.get_unchecked(5);
        let g: u8 = *self.square.get_unchecked(6);
        let h: u8 = *self.square.get_unchecked(7);
        let i: u8 = *self.square.get_unchecked(8);
        let j: u8 = *self.square.get_unchecked(9);
        let k: u8 = *self.square.get_unchecked(10);
        let l: u8 = *self.square.get_unchecked(11);
        let m: u8 = *self.square.get_unchecked(12);
        let n: u8 = *self.square.get_unchecked(13);
        let o: u8 = *self.square.get_unchecked(14);
        let p: u8 = *self.square.get_unchecked(15);

        let s1 = a + b + c + d;
        let s2 = a + e + i + m;
        let s3 = a + f + k + p;
        let s4 = b + f + j + n;
        let s5 = c + g + k + o;
        let s6 = d + g + j + m;
        let s7 = e + f + g + h;
        let s8 = i + j + k + l;

        match [s1, s2, s3, s4, s5, s6, s7, s8]
            .into_iter()
            .all(|sum| sum == OrderFour::MAGIC_SUM as u8)
        {
            true => Some(*self),
            false => None,
        }
    }

    /// Checks if a Permutation of element type T, order N is magic.
    /// Computations are scalar.
    pub fn check_s(&self) -> Option<Permutation<OrderFour>> {
        let a: u8 = *self.square.get(0).unwrap();
        let b: u8 = *self.square.get(1).unwrap();
        let c: u8 = *self.square.get(2).unwrap();
        let d: u8 = *self.square.get(3).unwrap();
        let e: u8 = *self.square.get(4).unwrap();
        let f: u8 = *self.square.get(5).unwrap();
        let g: u8 = *self.square.get(6).unwrap();
        let h: u8 = *self.square.get(7).unwrap();
        let i: u8 = *self.square.get(8).unwrap();
        let j: u8 = *self.square.get(9).unwrap();
        let k: u8 = *self.square.get(10).unwrap();
        let l: u8 = *self.square.get(11).unwrap();
        let m: u8 = *self.square.get(12).unwrap();
        let n: u8 = *self.square.get(13).unwrap();
        let o: u8 = *self.square.get(14).unwrap();
        let p: u8 = *self.square.get(15).unwrap();

        let s1 = a + b + c + d;
        let s2 = a + e + i + m;
        let s3 = a + f + k + p;
        let s4 = b + f + j + n;
        let s5 = c + g + k + o;
        let s6 = d + g + j + m;
        let s7 = e + f + g + h;
        let s8 = i + j + k + l;

        match [s1, s2, s3, s4, s5, s6, s7, s8]
            .into_iter()
            .all(|sum| sum == OrderFour::MAGIC_SUM as u8)
        {
            true => Some(*self),
            false => None,
        }
    }
}

#[cfg(test)]
mod check_tests {
    use super::*;

    #[test]
    fn check_unsafe_scalar() {
        let a = Permutation::<OrderThree>::kth(69074);
        let r = unsafe { a.check_s_unsafe() };
        assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
    }

    #[test]
    fn check_safe_scalar() {
        let a = Permutation::<OrderThree>::kth(69074);
        let r = a.check_s();
        assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
    }
}
