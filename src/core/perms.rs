use crate::{params::Params, OrderThree, Square};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Permutation<T: Copy + Clone, const N: usize> {
    pub square: Square<T, N>,
    pub index: usize,
}

impl Permutation<u8, { OrderThree::ELEMENTS }> {
    pub fn first() -> Permutation<u8, { OrderThree::ELEMENTS }> {
        let mut arr: [u8; OrderThree::ELEMENTS] = [0; OrderThree::ELEMENTS];
        for (elem, val) in arr.iter_mut().zip(1..=OrderThree::ELEMENTS as u8) {
            *elem = val;
        }
        Permutation {
            square: Square(arr),
            index: 0,
        }
    }

    pub fn kth(k: usize) -> Permutation<u8, { OrderThree::ELEMENTS }> {
        let mut n = Permutation::<u8, { OrderThree::ELEMENTS }>::first();
        let mut indeces = [0; { OrderThree::ELEMENTS }];

        let mut divisor = 1;
        for place in 1..OrderThree::ELEMENTS + 1 {
            if k / divisor == 0 {
                break;
            }
            indeces[OrderThree::ELEMENTS - place] = (k / divisor) % place;
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

pub trait NextPerm<T: Copy, const N: usize> {
    fn next_perm(&mut self) -> Option<&mut Permutation<T, { OrderThree::ELEMENTS }>>;
}

impl<T: Copy + PartialOrd> NextPerm<T, { OrderThree::ELEMENTS }>
    for Permutation<T, { OrderThree::ELEMENTS }>
{
    fn next_perm(&mut self) -> Option<&mut Permutation<T, { OrderThree::ELEMENTS }>> {
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
    use crate::{NextPerm, OrderThree, Params, Permutation};

    #[test]
    fn test_first() {
        let a = Permutation::<u8, { OrderThree::ELEMENTS }>::first();
        println!("{:?}", &a);
    }

    #[test]
    fn test_next() {
        let mut a = Permutation::<u8, { OrderThree::ELEMENTS }>::first();
        a = *a.next_perm().unwrap();
        println!("{:?}", &a);
    }

    #[test]
    fn test_kth() {
        let a = Permutation::<u8, { OrderThree::ELEMENTS }>::kth(0);
        println!("{:?}", a);

        let a = Permutation::<u8, { OrderThree::ELEMENTS }>::kth(1);
        println!("{:?}", &a);

        let a = Permutation::<u8, { OrderThree::ELEMENTS }>::kth(OrderThree::PERMUTATIONS - 2);
        println!("{:?}", &a);

        let a = Permutation::<u8, { OrderThree::ELEMENTS }>::kth(OrderThree::PERMUTATIONS - 1);
        println!("{:?}", &a);
    }
}
