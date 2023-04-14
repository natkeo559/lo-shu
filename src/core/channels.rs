use crate::{Check, Params, Permutation, OrderThree, OrderFour};
use std::sync::{atomic::AtomicBool, atomic::Ordering::Relaxed, mpsc::Sender, Arc};


pub trait Worker<P: Params> {
    fn channel_check(&self, start: usize, sender: Sender<Permutation<P>>, found: Arc<AtomicBool>) where [(); P::ELEMENTS]: ;
}

pub struct ThreadManager {
    pub threads: usize,
    pub poll: bool,
    pub polling_rate: usize,
}


impl Worker<OrderThree> for ThreadManager{
    fn channel_check(&self, start: usize, sender: Sender<Permutation<OrderThree>>, found: Arc<AtomicBool>) where [(); <OrderThree as Params>::ELEMENTS]: {
        for (it, n) in (start..OrderFour::PERMUTATIONS).step_by(self.threads).enumerate() {
            if let Some(sol) = Permutation::<OrderThree>::kth(n).check() {
                found.store(true, Relaxed);
                sender.send(sol).unwrap();
                return;
            } else if self.poll && it % self.polling_rate == 0 && found.load(Relaxed) {
                return;
            };
        }
    }
} 


impl Worker<OrderFour> for ThreadManager{
    fn channel_check(&self, start: usize, sender: Sender<Permutation<OrderFour>>, found: Arc<AtomicBool>) where [(); <OrderFour as Params>::ELEMENTS]: {
        let mut it = 0;
        for n in (start..OrderFour::PERMUTATIONS).step_by(self.threads) {
            if let Some(sol) = Permutation::<OrderFour>::kth(n).check() {
                found.store(true, Relaxed);
                sender.send(sol).unwrap();
                return;
            } else if self.poll && it % 16 == 0 && found.load(Relaxed) {
                return;
            };
            it += 1;
        }
    }
} 
