use crate::{CheckVector, O3, Params, Permutation, Enumerable, O4};
use std::{
    marker::PhantomData,
    sync::{atomic::AtomicBool, atomic::Ordering::Relaxed, mpsc::Sender, Arc},
};

pub trait Worker<P: Params> {
    fn channel_check(&self, start: usize, sender: Sender<Permutation<P>>, found: Arc<AtomicBool>)
    where
        [(); P::ELEMENTS]:;
}

pub struct ThreadManager<P: Params> {
    pub threads: usize,
    pub polling_rate: usize,
    pub one_stop: bool,
    phantom: PhantomData<P>,
}

impl<P: Params> ThreadManager<P> {
    pub fn new(th: usize, pr: usize, one: bool) -> Self {
        ThreadManager {
            threads: th,
            polling_rate: pr,
            one_stop: one,
            phantom: PhantomData,
        }
    }
}

macro_rules! impl_worker_for_tmgr {
    ($p:tt, $u:literal) => {
        impl Worker<$p> for ThreadManager<$p> {
            fn channel_check(
                &self,
                start: usize,
                sender: Sender<Permutation<$p>>,
                found: Arc<AtomicBool>,
            ) {
                for (count, n) in (start..$u).step_by(self.threads).enumerate() {
                    if let Some(sol) = Permutation::<$p>::kth(n.try_into().unwrap()).check_v() {
                        found.store(self.one_stop, Relaxed);
                        match sender.send(sol) {
                            Ok(_) => {}
                            Err(_) => {}
                        };
                    } else if count % self.polling_rate == 0 && found.load(Relaxed) {
                        return;
                    };
                }
                return;
            }
        }
    };
}

impl_worker_for_tmgr!(O3, 362880);
impl_worker_for_tmgr!(O4, 20922789888000);
