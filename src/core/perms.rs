use crate::{OrderThree, Params, Square};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Permutation<T: Copy + Clone, const N: usize> {
    pub square: Square<T, N>,
    pub index: usize,
}

impl Permutation<u8, 9> {
    pub fn first() -> Permutation<u8, 9> {
        let mut arr: [u8; 9] = [0; 9];
        for (elem, val) in arr.iter_mut().zip(1..=9_u8) {
            *elem = val % OrderThree::MODULUS as u8;
        }
        Permutation {
            square: Square(arr),
            index: 0,
        }
    }

    pub fn kth(k: usize) -> Permutation<u8, 9> {
        let mut n = Permutation::<u8, 9>::first();
        let mut indeces = [0; 9];

        let mut divisor = 1;
        for place in 1..9 + 1 {
            if k / divisor == 0 {
                break;
            }
            indeces[9 - place] = (k / divisor) % place;
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

pub trait Perms<T: Copy + PartialOrd, const N: usize> {
    fn next_perm(&mut self) -> Option<&mut Permutation<T, 9>>;
}

impl Perms<u8, 9> for Permutation<u8, 9> {
    fn next_perm(&mut self) -> Option<&mut Permutation<u8, 9>> {
        // Find non-increasing suffix
        let mut i: usize = self.square.len() - 1;
        while i > 0 && self.square[i - 1] >= self.square[i] {
            i -= 1;
        }
        if i == 0 {
            return None;
        }

        // Find successor to pivot
        let mut j: usize = self.square.len() - 1;
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
    use crate::{OrderThree, Params, Perms, Permutation};

    #[test]
    fn test_first() {
        let a = Permutation::<u8, 9>::first();
        println!("{:?}", &a);
    }

    #[test]
    fn test_next() {
        let mut a = Permutation::<u8, 9>::first();
        a = *a.next_perm().unwrap();
        println!("{:?}", &a);
    }

    #[test]
    fn test_kth() {
        let a = Permutation::<u8, 9>::kth(0);
        println!("{:?}", a);

        let a = Permutation::<u8, 9>::kth(1);
        println!("{:?}", &a);

        let a = Permutation::<u8, 9>::kth(OrderThree::PERMUTATIONS - 2);
        println!("{:?}", &a);

        let a = Permutation::<u8, 9>::kth(OrderThree::PERMUTATIONS - 1);
        println!("{:?}", &a);
    }

    #[test]
    fn test_id() {
        // let mut a = Permutation::<u8, 9>::first();
        // println!("{:?}", a.perm_id());

        // let mut a = Permutation::<u8, 9>::kth(OrderThree::PERMUTATIONS - 1);
        // println!("{:?}", a.perm_id());

        // let mut a = Permutation::<u8, 9>::kth(499);
        // println!("{:?}", a.perm_id());
    }
}
