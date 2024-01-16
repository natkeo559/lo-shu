use crate::{Construction, Params};
use rand::{self, seq::SliceRandom};

impl<P: Params + Copy> Construction<P>
where
    [(); P::ELEMENTS]:,
{
    /// Shuffles the elements of the construction's square using the Fisher-Yates algorithm.
    pub fn shuffle(&mut self) -> &mut Self {
        let mut rng = rand::thread_rng();
        self.square.data.shuffle(&mut rng);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::O3;

    use super::*;

    #[test]
    fn test_shuffle() {
        let mut a = Construction::<O3>::identity();
        a.shuffle();

        assert_ne!(a, Construction::<O3>::identity());
    }
}
