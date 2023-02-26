use crate::{Square, params::Params};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Permutation<T: Copy + Clone, P: Params> where [(); P::ELEMENTS]: {
    pub square: Square<T, P>,
    pub index: usize,
}

impl<P: Params> Permutation<u8, P> where [(); P::ELEMENTS]:{
    pub fn first() -> Permutation<u8, P> {
        let mut arr: [u8; P::ELEMENTS] = [0; P::ELEMENTS];
        for (elem, val) in arr.iter_mut().zip(1..=P::ELEMENTS as u8) {
            *elem = val;
        }
        Permutation {
            square: Square (arr),
            index: 0,
        }
    }

    pub fn kth(k: usize) -> Permutation<u8, P> {
        let mut n = Permutation::<u8, P>::first();
        let mut indeces = [0; P::ELEMENTS];

        let mut divisor = 1;
        for place in 1..P::ELEMENTS + 1 {
            if k / divisor == 0 {
                break;
            }
            indeces[P::ELEMENTS - place] = (k / divisor) % place;
            divisor *= place;
        }
        for (i, item) in indeces.iter().enumerate() {
            let index = item + i;
            if index != i {
                let temp = n.square[index];
                let mut j = index;
                while j > i {
                    n.square[j] = n.square[j - 1];
                    j -= 1;
                }
                n.square[i] = temp;
            }
        }
        Permutation {
            square: n.square,
            index: k,
        }
    }
}

pub trait NextPerm<T: Copy, P: Params> where [(); P::ELEMENTS]:{
    fn next_perm(&mut self) -> Option<&mut Permutation<T, P>>;
}

impl<T: Copy + PartialOrd, P: Params> NextPerm<T, P> for Permutation<T, P> where [(); P::ELEMENTS]:{
    fn next_perm(&mut self) -> Option<&mut Permutation<T, P>> {
        // Find non-increasing suffix
        let mut i: usize = P::ELEMENTS - 1;
        while i > 0 && self.square[i - 1] >= self.square[i] {
            i -= 1;
        }
        if i == 0 {
            return None;
        }

        // Find successor to pivot
        let mut j: usize = P::ELEMENTS - 1;
        while self.square[j] <= self.square[i - 1] {
            j -= 1;
        }
        self.square.swap(i - 1, j);

        // Reverse suffix
        self.square[i..].reverse();
        self.index += 1;
        Some(self)
    }
}

#[cfg(test)]
mod test_perms3 {
    use crate::{NextPerm, Permutation, OrderThree, Params};

    #[test]
    fn test_first() {
        let a = Permutation::<u8, OrderThree>::first();
        println!("{:?}", &a);
    }

    #[test]
    fn test_next() {
        let mut a = Permutation::<u8, OrderThree>::first();
        a = *a.next_perm().unwrap();
        println!("{:?}", &a);
    }

    #[test]
    fn test_kth() {
        let a = Permutation::<u8, OrderThree>::kth(0);
        println!("{:?}", a);

        let a = Permutation::<u8, OrderThree>::kth(1);
        println!("{:?}", &a);

        let a = Permutation::<u8, OrderThree>::kth(OrderThree::PERMUTATIONS - 2);
        println!("{:?}", &a);

        let a = Permutation::<u8, OrderThree>::kth(OrderThree::PERMUTATIONS - 1);
        println!("{:?}", &a);
    }
}
