use crate::{Construction, Params, Permutation, Square, VecSquare, O3, O4};
pub trait CheckScalar {
    type Output;

    /// Check if a square-like object is magic.
    ///
    /// # Safety
    /// Use of `get_unchecked` is unsafe. For a safe abstraction, use `get` to return references to Square elements.
    unsafe fn check_s_unsafe(&self) -> Option<Self::Output>;

    /// Check if a square-like object is magic.
    fn check_s(&self) -> Option<Self::Output>;
}

impl CheckScalar for Square<O3> {
    type Output = Self;

    #[inline]
    unsafe fn check_s_unsafe(&self) -> Option<Self::Output> {
        let a: u32 = *self.get_unchecked(0);
        let b: u32 = *self.get_unchecked(1);
        let c: u32 = *self.get_unchecked(2);
        let d: u32 = *self.get_unchecked(3);
        let e: u32 = *self.get_unchecked(4);
        let f: u32 = *self.get_unchecked(5);
        let g: u32 = *self.get_unchecked(6);
        let h: u32 = *self.get_unchecked(7);
        let i: u32 = *self.get_unchecked(8);

        let s1 = a + b + c;
        let s2 = a + e + i;
        let s3 = a + d + g;
        let s4 = b + e + h;
        let s5 = c + f + i;
        let s6 = c + e + g;
        let s7 = d + e + f;
        let s8 = g + h + i;

        [s1, s2, s3, s4, s5, s6, s7, s8]
            .into_iter()
            .all(|sum| sum == O3::MAGIC_SUM)
            .then_some(*self)
    }

    /// # Panics
    /// - If the value returned by `get` is `None`
    #[inline]
    fn check_s(&self) -> Option<Self::Output> {
        let a: u32 = *self.get(0).unwrap();
        let b: u32 = *self.get(1).unwrap();
        let c: u32 = *self.get(2).unwrap();
        let d: u32 = *self.get(3).unwrap();
        let e: u32 = *self.get(4).unwrap();
        let f: u32 = *self.get(5).unwrap();
        let g: u32 = *self.get(6).unwrap();
        let h: u32 = *self.get(7).unwrap();
        let i: u32 = *self.get(8).unwrap();

        let s1 = a + b + c;
        let s2 = a + e + i;
        let s3 = a + d + g;
        let s4 = b + e + h;
        let s5 = c + f + i;
        let s6 = c + e + g;
        let s7 = d + e + f;
        let s8 = g + h + i;

        [s1, s2, s3, s4, s5, s6, s7, s8]
            .into_iter()
            .all(|sum| sum == O3::MAGIC_SUM)
            .then_some(*self)
    }
}

impl CheckScalar for VecSquare<O3> {
    type Output = Self;

    #[inline]
    unsafe fn check_s_unsafe(&self) -> Option<Self::Output> {
        let a: u32 = *self.data.get_unchecked(0);
        let b: u32 = *self.data.get_unchecked(1);
        let c: u32 = *self.data.get_unchecked(2);
        let d: u32 = *self.data.get_unchecked(3);
        let e: u32 = *self.data.get_unchecked(4);
        let f: u32 = *self.data.get_unchecked(5);
        let g: u32 = *self.data.get_unchecked(6);
        let h: u32 = *self.data.get_unchecked(7);
        let i: u32 = *self.data.get_unchecked(8);

        let s1 = a + b + c;
        let s2 = a + e + i;
        let s3 = a + d + g;
        let s4 = b + e + h;
        let s5 = c + f + i;
        let s6 = c + e + g;
        let s7 = d + e + f;
        let s8 = g + h + i;

        [s1, s2, s3, s4, s5, s6, s7, s8]
            .into_iter()
            .all(|sum| sum == O3::MAGIC_SUM)
            .then_some((*self).clone())
    }

    /// # Panics
    /// - If the value returned by `get` is `None`
    #[inline]
    fn check_s(&self) -> Option<Self::Output> {
        let a: u32 = *self.data.first().unwrap();
        let b: u32 = *self.data.get(1).unwrap();
        let c: u32 = *self.data.get(2).unwrap();
        let d: u32 = *self.data.get(3).unwrap();
        let e: u32 = *self.data.get(4).unwrap();
        let f: u32 = *self.data.get(5).unwrap();
        let g: u32 = *self.data.get(6).unwrap();
        let h: u32 = *self.data.get(7).unwrap();
        let i: u32 = *self.data.get(8).unwrap();

        let s1 = a + b + c;
        let s2 = a + e + i;
        let s3 = a + d + g;
        let s4 = b + e + h;
        let s5 = c + f + i;
        let s6 = c + e + g;
        let s7 = d + e + f;
        let s8 = g + h + i;

        [s1, s2, s3, s4, s5, s6, s7, s8]
            .into_iter()
            .all(|sum| sum == O3::MAGIC_SUM)
            .then_some((*self).clone())
    }
}

impl CheckScalar for Square<O4> {
    type Output = Self;

    #[inline]
    unsafe fn check_s_unsafe(&self) -> Option<Self> {
        let a: u32 = *self.get_unchecked(0);
        let b: u32 = *self.get_unchecked(1);
        let c: u32 = *self.get_unchecked(2);
        let d: u32 = *self.get_unchecked(3);
        let e: u32 = *self.get_unchecked(4);
        let f: u32 = *self.get_unchecked(5);
        let g: u32 = *self.get_unchecked(6);
        let h: u32 = *self.get_unchecked(7);
        let i: u32 = *self.get_unchecked(8);
        let j: u32 = *self.get_unchecked(9);
        let k: u32 = *self.get_unchecked(10);
        let l: u32 = *self.get_unchecked(11);
        let m: u32 = *self.get_unchecked(12);
        let n: u32 = *self.get_unchecked(13);
        let o: u32 = *self.get_unchecked(14);
        let p: u32 = *self.get_unchecked(15);

        let s1 = a + b + c + d;
        let s2 = a + e + i + m;
        let s3 = a + f + k + p;
        let s4 = b + f + j + n;
        let s5 = c + g + k + o;
        let s6 = d + g + j + m;
        let s7 = e + f + g + h;
        let s8 = i + j + k + l;

        [s1, s2, s3, s4, s5, s6, s7, s8]
            .into_iter()
            .all(|sum| sum == O4::MAGIC_SUM)
            .then_some(*self)
    }

    /// # Panics
    /// - If the value returned by `get` is `None`
    #[inline]
    fn check_s(&self) -> Option<Self> {
        let a: u32 = *self.get(0).unwrap();
        let b: u32 = *self.get(1).unwrap();
        let c: u32 = *self.get(2).unwrap();
        let d: u32 = *self.get(3).unwrap();
        let e: u32 = *self.get(4).unwrap();
        let f: u32 = *self.get(5).unwrap();
        let g: u32 = *self.get(6).unwrap();
        let h: u32 = *self.get(7).unwrap();
        let i: u32 = *self.get(8).unwrap();
        let j: u32 = *self.get(9).unwrap();
        let k: u32 = *self.get(10).unwrap();
        let l: u32 = *self.get(11).unwrap();
        let m: u32 = *self.get(12).unwrap();
        let n: u32 = *self.get(13).unwrap();
        let o: u32 = *self.get(14).unwrap();
        let p: u32 = *self.get(15).unwrap();

        let s1 = a + b + c + d;
        let s2 = a + e + i + m;
        let s3 = a + f + k + p;
        let s4 = b + f + j + n;
        let s5 = c + g + k + o;
        let s6 = d + g + j + m;
        let s7 = e + f + g + h;
        let s8 = i + j + k + l;

        [s1, s2, s3, s4, s5, s6, s7, s8]
            .into_iter()
            .all(|sum| sum == O4::MAGIC_SUM)
            .then_some(*self)
    }
}

impl CheckScalar for VecSquare<O4> {
    type Output = Self;

    #[inline]
    unsafe fn check_s_unsafe(&self) -> Option<Self> {
        let a: u32 = *self.data.get_unchecked(0);
        let b: u32 = *self.data.get_unchecked(1);
        let c: u32 = *self.data.get_unchecked(2);
        let d: u32 = *self.data.get_unchecked(3);
        let e: u32 = *self.data.get_unchecked(4);
        let f: u32 = *self.data.get_unchecked(5);
        let g: u32 = *self.data.get_unchecked(6);
        let h: u32 = *self.data.get_unchecked(7);
        let i: u32 = *self.data.get_unchecked(8);
        let j: u32 = *self.data.get_unchecked(9);
        let k: u32 = *self.data.get_unchecked(10);
        let l: u32 = *self.data.get_unchecked(11);
        let m: u32 = *self.data.get_unchecked(12);
        let n: u32 = *self.data.get_unchecked(13);
        let o: u32 = *self.data.get_unchecked(14);
        let p: u32 = *self.data.get_unchecked(15);

        let s1 = a + b + c + d;
        let s2 = a + e + i + m;
        let s3 = a + f + k + p;
        let s4 = b + f + j + n;
        let s5 = c + g + k + o;
        let s6 = d + g + j + m;
        let s7 = e + f + g + h;
        let s8 = i + j + k + l;

        [s1, s2, s3, s4, s5, s6, s7, s8]
            .into_iter()
            .all(|sum| sum == O4::MAGIC_SUM)
            .then_some((*self).clone())
    }

    /// # Panics
    /// - If the value returned by `get` is `None`
    #[inline]
    fn check_s(&self) -> Option<Self> {
        let a: u32 = *self.data.first().unwrap();
        let b: u32 = *self.data.get(1).unwrap();
        let c: u32 = *self.data.get(2).unwrap();
        let d: u32 = *self.data.get(3).unwrap();
        let e: u32 = *self.data.get(4).unwrap();
        let f: u32 = *self.data.get(5).unwrap();
        let g: u32 = *self.data.get(6).unwrap();
        let h: u32 = *self.data.get(7).unwrap();
        let i: u32 = *self.data.get(8).unwrap();
        let j: u32 = *self.data.get(9).unwrap();
        let k: u32 = *self.data.get(10).unwrap();
        let l: u32 = *self.data.get(11).unwrap();
        let m: u32 = *self.data.get(12).unwrap();
        let n: u32 = *self.data.get(13).unwrap();
        let o: u32 = *self.data.get(14).unwrap();
        let p: u32 = *self.data.get(15).unwrap();

        let s1 = a + b + c + d;
        let s2 = a + e + i + m;
        let s3 = a + f + k + p;
        let s4 = b + f + j + n;
        let s5 = c + g + k + o;
        let s6 = d + g + j + m;
        let s7 = e + f + g + h;
        let s8 = i + j + k + l;

        [s1, s2, s3, s4, s5, s6, s7, s8]
            .into_iter()
            .all(|sum| sum == O4::MAGIC_SUM)
            .then_some((*self).clone())
    }
}

#[inline]
fn sum_constraint_vectors<P: Params>(values: &[u32]) -> Option<()>
where
    [(); P::ORDER]:,
{
    let mut chunks = values.chunks_exact(P::ORDER);
    assert!(chunks.remainder().is_empty());

    if chunks.len() == 1 {
        return if chunks.next()?.iter().sum::<u32>() == P::MAGIC_SUM {
            Some(())
        } else {
            None
        };
    }

    let sums = chunks.fold([0; P::ORDER], |mut acc, chunk| {
        for i in 0..P::ORDER {
            acc[i] += chunk[i];
        }
        acc
    });

    if sums.into_iter().all(|sum| sum == P::MAGIC_SUM) {
        Some(())
    } else {
        None
    }
}

// Reduce code duplication
//-------------------------------------------------------------------------------------------------

macro_rules! impl_generic_scalar_checker_for_type {
    ($t:tt) => {
        impl<P: Params + Copy> $t<P>
        where
            [(); P::ELEMENTS]:,
            [(); P::ORDER]:,
        {
            /// Check if a square-like object of any size is magic.
            #[inline]
            #[must_use]
            pub fn check_n_s(&self) -> Option<$t<P>> {
                let (r, c): (Vec<u32>, Vec<u32>) = (0..P::ELEMENTS)
                    .map(|e| e / P::ORDER)
                    .zip((0usize..P::ELEMENTS).map(|s| s % P::ORDER))
                    .map(|(i, a)| {
                        (
                            self.square.data[i * P::ORDER + a],
                            self.square.data[a * P::ORDER + i],
                        )
                    })
                    .unzip();

                let t1: Vec<u32> = (0..P::ORDER)
                    .map(|a| self.square.data[a * (P::ORDER + 1)])
                    .collect();
                let t2: Vec<u32> = (0..P::ORDER)
                    .map(|a| self.square.data[(a + 1) * (P::ORDER - 1)])
                    .collect();

                sum_constraint_vectors(&r)?;
                sum_constraint_vectors(&c)?;
                sum_constraint_vectors(&t1)?;
                sum_constraint_vectors(&t2)?;

                Some((*self).clone())
            }
        }
    };
}

impl_generic_scalar_checker_for_type!(Construction);
impl_generic_scalar_checker_for_type!(Permutation);

//-------------------------------------------------------------------------------------------------

// Reduce code duplication
//-------------------------------------------------------------------------------------------------
macro_rules! impl_check_scalar_for_type_with_param {
    ($type:tt, $param:tt) => {
        impl CheckScalar for $type<$param> {
            type Output = Self;

            #[inline]
            #[must_use]
            unsafe fn check_s_unsafe(&self) -> Option<Self::Output> {
                let p = self.clone();

                if p.square.check_s_unsafe().is_some() {
                    return Some(p);
                }

                None
            }

            /// # Panics
            /// - If the value returned by `get` is `None`
            #[inline]
            #[must_use]
            fn check_s(&self) -> Option<Self::Output> {
                let p = self.clone();

                if p.square.check_s().is_some() {
                    return Some(p);
                }

                None
            }
        }
    };
}

impl_check_scalar_for_type_with_param!(Permutation, O3);
impl_check_scalar_for_type_with_param!(Permutation, O4);
impl_check_scalar_for_type_with_param!(Construction, O3);
impl_check_scalar_for_type_with_param!(Construction, O4);

//-------------------------------------------------------------------------------------------------

#[cfg(test)]
mod check_tests {
    use super::*;
    use crate::{Enumerable, ParameterSetError};

    #[test]
    fn test_safe_3() -> Result<(), ParameterSetError> {
        let a1 = Permutation::<O3>::kth(69074);
        let a2 = Permutation::<O3>::kth(69075);
        let b1 = Construction::try_from(a1)?;
        let b2 = Construction::try_from(a2)?;
        let a1_result = a1.check_s();
        let a2_result = a2.check_s();
        let b1_result = b1.check_s();
        let b2_result = b2.check_s();

        assert_eq!(Some(a1), a1_result);
        assert_eq!(None, a2_result);
        assert_eq!(Some(b1), b1_result);
        assert_eq!(None, b2_result);

        Ok(())
    }

    #[test]
    fn test_unsafe_3() -> Result<(), ParameterSetError> {
        let a1 = Permutation::<O3>::kth(69074);
        let a2 = Permutation::<O3>::kth(69075);
        let b1 = Construction::try_from(a1)?;
        let b2 = Construction::try_from(a2)?;
        let a1_result = unsafe { a1.check_s_unsafe() };
        let a2_result = unsafe { a2.check_s_unsafe() };
        let b1_result = unsafe { b1.check_s_unsafe() };
        let b2_result = unsafe { b2.check_s_unsafe() };

        assert_eq!(Some(a1), a1_result);
        assert_eq!(None, a2_result);
        assert_eq!(Some(b1), b1_result);
        assert_eq!(None, b2_result);

        Ok(())
    }

    #[test]
    fn test_check_n_s_3() -> Result<(), ParameterSetError> {
        let a1 = Permutation::<O3>::kth(69074);
        let a2 = Permutation::<O3>::kth(69075);
        let b1 = Construction::try_from(a1)?;
        let b2 = Construction::try_from(a2)?;
        let a1_result = a1.check_n_s();
        let a2_result = a2.check_n_s();
        let b1_result = b1.check_n_s();
        let b2_result = b2.check_n_s();

        assert_eq!(Some(a1), a1_result);
        assert_eq!(None, a2_result);
        assert_eq!(Some(b1), b1_result);
        assert_eq!(None, b2_result);

        Ok(())
    }

    #[test]
    fn test_check_n_s_4() -> Result<(), ParameterSetError> {
        let a1 = Permutation::<O4>::kth(80867885530);
        let a2 = Permutation::<O4>::kth(80867885531);
        let b1 = Construction::try_from(a1)?;
        let b2 = Construction::try_from(a2)?;
        let a1_result = a1.check_n_s();
        let a2_result = a2.check_n_s();
        let b1_result = b1.check_n_s();
        let b2_result = b2.check_n_s();

        assert_eq!(Some(a1), a1_result);
        assert_eq!(None, a2_result);
        assert_eq!(Some(b1), b1_result);
        assert_eq!(None, b2_result);

        Ok(())
    }
}
