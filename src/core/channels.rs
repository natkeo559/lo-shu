use crate::{Enumerable, Params, Permutation, O3, O4, O5};
use std::{
    fmt::Debug,
    marker::PhantomData,
    sync::{atomic::AtomicBool, atomic::Ordering::Relaxed, mpsc::{self, Sender}, Arc}, thread,
    collections::BTreeSet,
};

pub trait Worker<P: Params> {
    fn channel_check(&self, start: u128, sender: Sender<Permutation<P>>, found: Arc<AtomicBool>)
    where
        [(); P::ELEMENTS]:;
}

pub struct ThreadManager<P: Params> {
    pub threads: u128,
    pub polling_rate: usize,
    pub one_stop: bool,
    phantom: PhantomData<P>,
}

impl<P: Params> ThreadManager<P> {
    pub fn new(th: u128, pr: usize, one: bool) -> Self {
        ThreadManager {
            threads: th,
            polling_rate: pr,
            one_stop: one,
            phantom: PhantomData,
        }
    }
}

macro_rules! impl_worker_for_tmgr {
    ($p:tt,$e:ty,$u:literal) => {
        impl Worker<$p> for ThreadManager<$p> {
            fn channel_check(
                &self,
                start: u128,
                sender: Sender<Permutation<$p>>,
                found: Arc<AtomicBool>,
            ) {
                for (count, n) in (start as $e..$u).step_by(self.threads as usize).enumerate() {
                    if let Some(sol) = Permutation::<$p>::kth(n.try_into().unwrap()).check_n_s() {
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

impl_worker_for_tmgr!(O3, u32, 362880);
impl_worker_for_tmgr!(O4, u64, 20922789888000);
impl_worker_for_tmgr!(O5, u128, 15511210043330985984000000);

#[derive(Debug)]
pub struct Yes;

#[derive(Debug)]
pub struct No;

pub trait ToAssign: Debug {}
pub trait Assigned: ToAssign {}
pub trait NotAssigned: ToAssign {}

impl ToAssign for Yes {}
impl ToAssign for No {}

impl Assigned for Yes {}
impl NotAssigned for No {}

#[derive(Debug, Clone)]
pub struct MessageSolverBuilder<P: Params, T, ThreadsSet, UpperSet, PollingSet, FirstSet>
where
    ThreadsSet: ToAssign,
    UpperSet: ToAssign,
    PollingSet: ToAssign,
    FirstSet: ToAssign,
{
    threads: usize,
    upper_bound: T,
    polling_rate: usize,
    find_first: bool,
    echo: bool,
    gen_d: bool,
    phantom: PhantomData<P>,
    t_set_phantom: PhantomData<ThreadsSet>,
    u_set_phantom: PhantomData<UpperSet>,
    p_set_phantom: PhantomData<PollingSet>,
    f_set_phantom: PhantomData<FirstSet>,
}

impl<P: Params, T, ThreadsSet, UpperSet, PollingSet, FirstSet> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, FirstSet>
where
    ThreadsSet: ToAssign,
    UpperSet: ToAssign,
    PollingSet: ToAssign,
    FirstSet: ToAssign,
{
    pub fn threads(self, threads: usize) -> MessageSolverBuilder<P, T, Yes, UpperSet, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first: self.find_first,
            echo: self.echo,
            gen_d: self.gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    pub fn upper_bound(self, upper_bound: T) -> MessageSolverBuilder<P, T, ThreadsSet, Yes, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound,
            polling_rate: self.polling_rate,
            find_first: self.find_first,
            echo: self.echo,
            gen_d: self.gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    pub fn polling_rate(self, polling_rate: usize) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, Yes, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate,
            echo: self.echo,
            gen_d: self.gen_d,
            find_first: self.find_first,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    pub fn find_first(self, find_first: bool) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, Yes> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first,
            echo: self.echo,
            gen_d: self.gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    pub fn echo(self, echo: bool) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first: self.find_first,
            echo,
            gen_d: self.gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    pub fn generate_d(self, gen_d: bool) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first: self.find_first,
            echo: self.echo,
            gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }
}

pub struct MessageSolver<P: Params> {
    param_phantom: PhantomData<P>,
}

macro_rules! impl_message_solver {
    ($p:tt, $t:ty) => {
        impl MessageSolver<$p> {
            pub fn default() -> MessageSolverBuilder<$p, $t, No, No, No, No> {
                MessageSolverBuilder {
                    threads: 1,
                    upper_bound: 1,
                    polling_rate: 1,
                    find_first: false,
                    echo: false,
                    gen_d: false,
                    phantom: PhantomData {},
                    t_set_phantom: PhantomData {},
                    u_set_phantom: PhantomData {},
                    p_set_phantom: PhantomData {},
                    f_set_phantom: PhantomData {},
                }
            }
        }
    };
}

macro_rules! impl_message_solver_builder {
    ($p:tt, $t:ty) => {
        impl_message_solver!($p, $t);

        // threads, upper, no poll, no first
        impl MessageSolverBuilder<$p, $t, Yes, Yes, No, No> {
            pub fn execute(self) -> Result<(), anyhow::Error> {
                let (sx, rx) = mpsc::channel();
        
                for i in 0..self.threads {
                    let sender: Sender<Permutation<$p>> = sx.clone();
                    thread::spawn(move || {
                        for n in (i as $t..self.upper_bound).step_by(self.threads) {
                            if let Some(sol) = Permutation::<$p>::kth(n.try_into().unwrap()).check_n_s() {
                                match sender.send(sol) {
                                    Ok(_) => {}
                                    Err(_) => {}
                                };
                            }
                        }
                        return;
                    });
                }
                
                for _ in 0..8 {
                    let idxs =  rx.recv()?;
                    let data = idxs.generate_d()
                            .into_iter()
                            .map(|a| a.clone().index()).collect::<BTreeSet<$t>>();

                    if self.echo {
                        for i in data {
                            println!("{}", i)
                        }
                    };
                }

                Ok(())
            }
        }
        
        impl MessageSolverBuilder<$p, $t, Yes, Yes, Yes, Yes>
        where
            [(); $p::ELEMENTS]:
        {
            pub fn execute(self) -> BTreeSet<$t> {
                let f = Arc::new(AtomicBool::new(false));
                let (sx, rx) = mpsc::channel();
        
                for i in 0..self.threads {
                    let sender: Sender<Permutation<$p>> = sx.clone();
                    let found = f.clone();
                    thread::spawn(move || {
                        for (count, n) in (i as $t..self.upper_bound).step_by(self.threads).enumerate() {
                            if let Some(sol) = Permutation::<$p>::kth(n.try_into().unwrap()).check_n_s() {
                                found.store(self.find_first, Relaxed);
                                match sender.send(sol) {
                                    Ok(_) => {return}
                                    Err(_) => {}
                                };
                            } else if count % self.polling_rate == 0 && found.load(Relaxed) {
                                return;
                            };
                        }
                        return;
                    });
                }
                
                match rx.recv() {
                    Ok(idxs) => {
                        let ret = idxs.generate_d()
                                .into_iter()
                                .map(|a| a.clone().index()).collect::<BTreeSet<$t>>();

                        if self.echo {
                            println!("{}", idxs)
                        }

                        ret
                    }
                    Err(_) => panic!("Worker threads disconnected before solution found!"),
                }
            }
        }
    };
}

impl_message_solver_builder!(O3, u32);
impl_message_solver_builder!(O4, u64);
impl_message_solver_builder!(O5, u128);


#[cfg(test)]
mod channels_tests {
    // use super::*;

    use crate::{O3, MessageSolver};

    #[test]
    fn test_builder() -> Result<(), anyhow::Error> {
        MessageSolver::<O3>::default().threads(16).upper_bound(362880).echo(true).generate_d(false).execute()?;
        // for i in a {
        //     println!("{}", Permutation::<O3>::kth(i))
        // }

        Ok(())
    }
}
