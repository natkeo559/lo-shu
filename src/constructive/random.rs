use crate::{Construction, Params};
use rand::{self, seq::SliceRandom};

impl<P: Params + Copy> Construction<P>
where
    [(); P::ELEMENTS]:,
{
    pub fn shuffle(&mut self) -> &mut Self {
        let mut rng = rand::thread_rng();
        self.square.0.shuffle(&mut rng);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::OrderThree;

    use super::*;

    #[test]
    fn test_shuffle() {
        let mut a = Construction::<OrderThree>::identity();
        a.shuffle();

        assert_ne!(a, Construction::<OrderThree>::identity());
    }
}
