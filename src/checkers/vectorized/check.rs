use itertools::Itertools;

use crate::{Construction, OrderFour, OrderThree, Params, Permutation, Square};
use std::simd::{Which::*, *};

pub trait CheckVector {
    type Output;

    /// Checks if a Permutation of element type T, order N is magic.
    /// Computations are vectorized and implemented using std::simd.
    ///
    /// # Safety
    ///
    /// Use of `get_unchecked` is unsafe. For a safe abstraction, use `get` to return references to Square elements.
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output>;

    /// Checks if a Permutation of element type T, order N is magic.
    /// Computations are vectorized and implemented using std::simd.
    fn check_v(&self) -> Option<Self::Output>;
}

impl CheckVector for Square<OrderThree> {
    type Output = Self;

    #[inline(always)]
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output> {
        const VMASK: Simd<u8, 8_usize> =
            Simd::from_slice(&[OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);

        let vals: [Simd<u8, 8_usize>; 2] = [
            Simd::from_slice(&[
                *self.get_unchecked(0),
                *self.get_unchecked(1),
                *self.get_unchecked(2),
                *self.get_unchecked(3),
                *self.get_unchecked(4),
                *self.get_unchecked(5),
                *self.get_unchecked(6),
                *self.get_unchecked(7),
            ]),
            Simd::from_slice(&[*self.get_unchecked(8), 0, 0, 0, 0, 0, 0, 0]),
        ];

        let mut v_a = simd_swizzle!(vals[0], [0, 3, 6, 0, 1, 2, 0, 2]);
        let mut v_b = simd_swizzle!(vals[0], [1, 4, 7, 3, 4, 5, 4, 4]);

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(2),
                First(5),
                Second(0),
                First(6),
                First(7),
                Second(0),
                Second(0),
                First(6)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        match v_a == VMASK {
            true => Some(*self),
            false => None,
        }
    }

    #[inline(always)]
    fn check_v(&self) -> Option<Self::Output> {
        const VMASK: Simd<u8, 8_usize> =
            Simd::from_slice(&[OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);

        let vals: [Simd<u8, 8_usize>; 2] = [
            Simd::from_slice(&[
                *self.get(0).unwrap(),
                *self.get(1).unwrap(),
                *self.get(2).unwrap(),
                *self.get(3).unwrap(),
                *self.get(4).unwrap(),
                *self.get(5).unwrap(),
                *self.get(6).unwrap(),
                *self.get(7).unwrap(),
            ]),
            Simd::from_slice(&[*self.get(8).unwrap(), 0, 0, 0, 0, 0, 0, 0]),
        ];

        let mut v_a = simd_swizzle!(vals[0], [0, 3, 6, 0, 1, 2, 0, 2]);
        let mut v_b = simd_swizzle!(vals[0], [1, 4, 7, 3, 4, 5, 4, 4]);

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(2),
                First(5),
                Second(0),
                First(6),
                First(7),
                Second(0),
                Second(0),
                First(6)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        match v_a == VMASK {
            true => Some(*self),
            false => None,
        }
    }
}

impl CheckVector for Square<OrderFour> {
    type Output = Self;

    #[inline(always)]
    unsafe fn check_v_unsafe(&self) -> Option<Self> {
        const VMASK: Simd<u8, 8_usize> =
            Simd::from_slice(&[OrderFour::MAGIC_SUM as u8; OrderFour::CONSTRAINT_VECTORS]);

        let vals: [Simd<u8, 8_usize>; 2] = [
            Simd::from_slice(&[
                *self.get_unchecked(0),
                *self.get_unchecked(1),
                *self.get_unchecked(2),
                *self.get_unchecked(3),
                *self.get_unchecked(4),
                *self.get_unchecked(5),
                *self.get_unchecked(6),
                *self.get_unchecked(7),
            ]),
            Simd::from_slice(&[
                *self.get_unchecked(8),
                *self.get_unchecked(9),
                *self.get_unchecked(10),
                *self.get_unchecked(11),
                *self.get_unchecked(12),
                *self.get_unchecked(13),
                *self.get_unchecked(14),
                *self.get_unchecked(15),
            ]),
        ];

        let mut v_a = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(0),
                First(0),
                First(0),
                First(1),
                First(2),
                First(3),
                First(4),
                Second(0)
            ]
        );
        let mut v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(1),
                First(4),
                First(5),
                First(5),
                First(6),
                First(6),
                First(5),
                Second(1)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(2),
                Second(0),
                Second(2),
                Second(1),
                Second(2),
                Second(1),
                First(6),
                Second(2)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(3),
                Second(4),
                Second(7),
                Second(5),
                Second(6),
                Second(4),
                First(7),
                Second(3)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        match v_a == VMASK {
            true => Some(*self),
            false => None,
        }
    }

    #[inline(always)]
    fn check_v(&self) -> Option<Self> {
        const VMASK: Simd<u8, 8_usize> =
            Simd::from_slice(&[OrderFour::MAGIC_SUM as u8; OrderFour::CONSTRAINT_VECTORS]);

        let vals: [Simd<u8, 8_usize>; 2] = [
            Simd::from_slice(&[
                *self.get(0).unwrap(),
                *self.get(1).unwrap(),
                *self.get(2).unwrap(),
                *self.get(3).unwrap(),
                *self.get(4).unwrap(),
                *self.get(5).unwrap(),
                *self.get(6).unwrap(),
                *self.get(7).unwrap(),
            ]),
            Simd::from_slice(&[
                *self.get(8).unwrap(),
                *self.get(9).unwrap(),
                *self.get(10).unwrap(),
                *self.get(11).unwrap(),
                *self.get(12).unwrap(),
                *self.get(13).unwrap(),
                *self.get(14).unwrap(),
                *self.get(15).unwrap(),
            ]),
        ];

        let mut v_a = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(0),
                First(0),
                First(0),
                First(1),
                First(2),
                First(3),
                First(4),
                Second(0)
            ]
        );
        let mut v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(1),
                First(4),
                First(5),
                First(5),
                First(6),
                First(6),
                First(5),
                Second(1)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(2),
                Second(0),
                Second(2),
                Second(1),
                Second(2),
                Second(1),
                First(6),
                Second(2)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(3),
                Second(4),
                Second(7),
                Second(5),
                Second(6),
                Second(4),
                First(7),
                Second(3)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        match v_a == VMASK {
            true => Some(*self),
            false => None,
        }
    }
}

impl CheckVector for Permutation<OrderThree> {
    type Output = Self;

    #[inline(always)]
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output> {
        let p = *self;

        if p.square.check_v_unsafe().is_some() {
            Some(p)
        } else {
            None
        }
    }

    #[inline(always)]
    fn check_v(&self) -> Option<Self::Output> {
        let p = *self;

        if p.square.check_v().is_some() {
            Some(p)
        } else {
            None
        }
    }
}

impl CheckVector for Permutation<OrderFour> {
    type Output = Self;

    #[inline(always)]
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output> {
        let p = *self;

        if p.square.check_v_unsafe().is_some() {
            Some(p)
        } else {
            None
        }
    }

    #[inline(always)]
    fn check_v(&self) -> Option<Self::Output> {
        let p = *self;

        if p.square.check_v().is_some() {
            Some(p)
        } else {
            None
        }
    }
}

impl CheckVector for Construction<OrderThree> {
    type Output = Self;

    #[inline(always)]
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output> {
        let p = *self;

        if p.square.check_v_unsafe().is_some() {
            Some(p)
        } else {
            None
        }
    }

    #[inline(always)]
    fn check_v(&self) -> Option<Self::Output> {
        let p = *self;

        if p.square.check_v().is_some() {
            Some(p)
        } else {
            None
        }
    }
}

impl CheckVector for Construction<OrderFour> {
    type Output = Self;

    #[inline(always)]
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output> {
        let p = *self;

        if p.square.check_v_unsafe().is_some() {
            Some(p)
        } else {
            None
        }
    }

    #[inline(always)]
    fn check_v(&self) -> Option<Self::Output> {
        let p = *self;

        if p.square.check_v().is_some() {
            Some(p)
        } else {
            None
        }
    }
}

impl<P: Params + Copy> Construction<P> where [(); P::ELEMENTS]:{
    

    pub fn check_n_v(&self) -> Option<Construction<P>>{
        let (r, c): (Vec<u8>, Vec<u8>) = (0..P::ELEMENTS)
            .map(|e| e / P::ORDER)
            .zip((0usize..P::ELEMENTS).map(|s| s % P::ORDER))
            .map(|(i, a)| (self.square[i * P::ORDER + a], self.square[a * P::ORDER + i]))
            .unzip();
        let rows = r.chunks_exact(P::ORDER).collect_vec();
        let cols = c.chunks_exact(P::ORDER).collect_vec();
        let t1: Vec<u8> = (0..P::ORDER).map(|a| self.square[a * (P::ORDER + 1)]).collect();
        let t2: Vec<u8> = (0..P::ORDER).map(|a| self.square[(a + 1) * (P::ORDER - 1)]).collect();

        let mut bytes: usize = 2usize.pow(((P::ORDER as f64).log10() / (2_f64).log10()).ceil() as u32);
        if bytes < 8 {
            bytes = 8;
        }
        let pad = bytes.abs_diff(P::CONSTRAINT_VECTORS);
        let mut buffs = vec![];
        for _ in 0..P::ORDER{
            buffs.push(Vec::<u8>::with_capacity(bytes))
        } 

        for i in 0..P::ORDER {
            for j in rows[i] {
                buffs[i].push(*j)
            }
            for j in cols[i] {
                buffs[i].push(*j)
            }
            buffs[i].push(t1[i]);
            buffs[i].push(t2[i]);

            for _ in 0..pad{
                buffs[i].push(0)
            }
        }

        let mut r_vec: Vec<u8> = vec![];
        for _ in 0..P::CONSTRAINT_VECTORS {
            r_vec.push(P::MAGIC_SUM as u8)
        }
        for _ in 0..pad {
            r_vec.push(0)
        }

        match bytes {
            8 => {
                let sumv: Simd<u8, 8> = Simd::from_slice(&r_vec[..]);
                let mut fv: Simd<u8, 8> = Simd::from_slice(buffs.iter().by_ref().next().unwrap()); 
                for i in buffs.into_iter().skip(1) {
                    fv += Simd::from_slice(&i[..])
                }
                if fv == sumv {
                    Some(*self)
                } else {
                    None
                }
            },
            16 => {
                let sumv: Simd<u8, 16> = Simd::from_slice(&r_vec[..]);
                let mut fv: Simd<u8, 16> = Simd::from_slice(buffs.iter().by_ref().next().unwrap()); 
                for i in buffs.into_iter().skip(1) {
                    fv += Simd::from_slice(&i[..])
                }
                if fv == sumv {
                    Some(*self)
                } else {
                    None
                }
            },
            32 => {
                let sumv: Simd<u8, 32> = Simd::from_slice(&r_vec[..]);
                let mut fv: Simd<u8, 32> = Simd::from_slice(buffs.iter().by_ref().next().unwrap()); 
                for i in buffs.into_iter().skip(1) {
                    fv += Simd::from_slice(&i[..])
                }
                if fv == sumv {
                    Some(*self)
                } else {
                    None
                }
            },
            64 => {
                let sumv: Simd<u8, 32> = Simd::from_slice(&r_vec[..]);
                let mut fv: Simd<u8, 32> = Simd::from_slice(buffs.iter().by_ref().next().unwrap()); 
                for i in buffs.into_iter().skip(1) {
                    fv += Simd::from_slice(&i[..])
                }
                if fv == sumv {
                    Some(*self)
                } else {
                    None
                }
            },
            a => panic!("Could not pick SIMD size! {a}")
        }
        

    }
} 

#[cfg(test)]
mod check_tests {
    use super::*;

    #[test]
    fn check_unchecked_single() {
        let a = Permutation::<OrderThree>::kth(69074);
        let r = unsafe { a.check_v_unsafe() };
        assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);

        let a = Permutation::<OrderFour>::kth(80867885530);

        println!("{}", a.square);

        let r = a.check_v();
        assert_eq!(Some(Permutation::<OrderFour>::kth(80867885530)), r);
    }

    #[test]
    fn check_single() {
        let a = Permutation::<OrderThree>::kth(69074);
        let r = a.check_v();
        assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
    }
}
