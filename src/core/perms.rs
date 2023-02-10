use crate::Square;

pub trait KthPerm<T> {
    ///In-place constructor for permutations of squares given 'k' from the lexicographically ordered set of permutations.
    fn kth_perm(&mut self, k: i32) -> Self;
}

impl<T: Clone + Copy> KthPerm<T> for Square<T> {
    fn kth_perm(&mut self, k: i32) -> Self {
        /*
        Based on:
        https://stackoverflow.com/questions/31216097/given-n-and-k-return-the-kth-permutation-sequence
        */

        let mut indeces = [0; 9];

        let mut divisor = 1;
        for place in 1..10 {
            if k / divisor == 0 {
                break;
            }
            indeces[9 - place] = (k / divisor) % place as i32;
            divisor *= place as i32;
        }
        for (i, index) in indeces.iter().enumerate() {
            //30% IMPROVEMENT!
            if index != &(i as i32) {
                let temp = self.array[*index as usize];
                let mut j = *index as usize;
                while j > i {
                    self.array[j] = self.array[j - 1];
                    j -= 1;
                }
                self.array[i] = temp;
            }
        }
        Self { array: self.array }
    }
}
