use flexi_logger::{FileSpec, Logger, WriteMode};
use log::Level;

use crate::{Enumerable, Params, Permutation, O3, O4, O5};
use std::{
    collections::BTreeSet,
    fmt::Debug,
    marker::PhantomData,
    path::PathBuf,
    sync::{
        atomic::AtomicBool,
        atomic::Ordering::Relaxed,
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Square,
    Index,
}

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

/// Builder for MPSC Message Solver
///
/// # Note:
///
/// Generic params:
///     - P: Params
///     - T: Integer type for UpperBound
///     - ThreadSet: threading enabled
///     - UpperSet: upper bound enabled
///     - PollingSet: polling enabled
///     - FirstSet: shared flag/thread breakout enabled
///
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
    path: PathBuf,
    n: u128,
    file_format: OutputFormat,
    stdout_format: OutputFormat,
    start: T,
    filename: String,
    echo: bool,
    gen_d: bool,
    phantom: PhantomData<P>,
    t_set_phantom: PhantomData<ThreadsSet>,
    u_set_phantom: PhantomData<UpperSet>,
    p_set_phantom: PhantomData<PollingSet>,
    f_set_phantom: PhantomData<FirstSet>,
}

impl<P: Params, T, ThreadsSet, UpperSet, PollingSet, FirstSet>
    MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, FirstSet>
where
    ThreadsSet: ToAssign,
    UpperSet: ToAssign,
    PollingSet: ToAssign,
    FirstSet: ToAssign,
{
    #[inline]
    pub fn threads(
        self,
        threads: usize,
    ) -> MessageSolverBuilder<P, T, Yes, UpperSet, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first: self.find_first,
            n: self.n,
            file_format: self.file_format,
            stdout_format: self.stdout_format,
            start: self.start,
            filename: self.filename,
            path: self.path,
            echo: self.echo,
            gen_d: self.gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    #[inline]
    pub fn upper_bound(
        self,
        upper_bound: T,
    ) -> MessageSolverBuilder<P, T, ThreadsSet, Yes, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound,
            polling_rate: self.polling_rate,
            file_format: self.file_format,
            find_first: self.find_first,
            n: self.n,
            start: self.start,
            filename: self.filename,
            stdout_format: self.stdout_format,
            path: self.path,
            echo: self.echo,
            gen_d: self.gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    #[inline]
    pub fn n(
        self,
        n: u128,
    ) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first: self.find_first,
            n,
            start: self.start,
            file_format: self.file_format,
            stdout_format: self.stdout_format,
            filename: self.filename,
            path: self.path,
            echo: self.echo,
            gen_d: self.gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    #[inline]
    pub fn polling_rate(
        self,
        polling_rate: usize,
    ) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, Yes, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate,
            echo: self.echo,
            start: self.start,
            gen_d: self.gen_d,
            stdout_format: self.stdout_format,
            file_format: self.file_format,
            find_first: self.find_first,
            n: self.n,
            filename: self.filename,
            path: self.path,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    #[inline]
    pub fn find_first(
        self,
        find_first: bool,
    ) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, Yes> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first,
            n: 1,
            stdout_format: self.stdout_format,
            filename: self.filename,
            path: self.path,
            file_format: self.file_format,
            start: self.start,
            echo: self.echo,
            gen_d: self.gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    #[inline]
    pub fn echo(
        self,
        echo: bool,
    ) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first: self.find_first,
            n: self.n,
            stdout_format: self.stdout_format,
            filename: self.filename,
            path: self.path,
            start: self.start,
            echo,
            file_format: self.file_format,
            gen_d: self.gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    #[inline]
    pub fn output_dir<S: Into<PathBuf>>(
        self,
        path: S,
    ) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first: self.find_first,
            n: self.n,
            filename: self.filename,
            path: path.into(),
            start: self.start,
            stdout_format: self.stdout_format,
            echo: self.echo,
            file_format: self.file_format,
            gen_d: self.gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    #[inline]
    pub fn filename<S: Into<String>>(
        self,
        filename: S,
    ) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first: self.find_first,
            n: self.n,
            file_format: self.file_format,
            stdout_format: self.stdout_format,
            start: self.start,
            filename: filename.into(),
            path: self.path,
            echo: self.echo,
            gen_d: self.gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    #[inline]
    pub fn start(
        self,
        index: T,
    ) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first: self.find_first,
            n: self.n,
            filename: self.filename,
            start: index,
            path: self.path,
            echo: self.echo,
            file_format: self.file_format,
            gen_d: self.gen_d,
            stdout_format: self.stdout_format,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    #[inline]
    pub fn file_format(
        self,
        file_format: OutputFormat,
    ) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first: self.find_first,
            n: self.n,
            filename: self.filename,
            start: self.start,
            path: self.path,
            stdout_format: self.stdout_format,
            echo: self.echo,
            file_format,
            gen_d: self.gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    #[inline]
    pub fn stdout_format(
        self,
        stdout_format: OutputFormat,
    ) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first: self.find_first,
            n: self.n,
            filename: self.filename,
            start: self.start,
            path: self.path,
            stdout_format,
            echo: self.echo,
            file_format: self.file_format,
            gen_d: self.gen_d,
            phantom: PhantomData {},
            t_set_phantom: PhantomData {},
            u_set_phantom: PhantomData {},
            p_set_phantom: PhantomData {},
            f_set_phantom: PhantomData {},
        }
    }

    #[inline]
    pub fn generate_d(
        self,
        gen_d: bool,
    ) -> MessageSolverBuilder<P, T, ThreadsSet, UpperSet, PollingSet, FirstSet> {
        MessageSolverBuilder {
            threads: self.threads,
            upper_bound: self.upper_bound,
            polling_rate: self.polling_rate,
            find_first: self.find_first,
            n: self.n,
            filename: self.filename,
            stdout_format: self.stdout_format,
            file_format: self.file_format,
            start: self.start,
            path: self.path,
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
            pub fn default_build() -> MessageSolverBuilder<$p, $t, No, No, No, No> {
                MessageSolverBuilder {
                    threads: 1,
                    upper_bound: 1,
                    polling_rate: 1,
                    find_first: false,
                    path: PathBuf::from("/examples/collected/"),
                    filename: "Output".to_string(),
                    start: 0,
                    n: 1,
                    file_format: OutputFormat::Index,
                    stdout_format: OutputFormat::Square,
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

fn default_format_index(
    write: &mut dyn std::io::Write,
    _now: &mut flexi_logger::DeferredNow,
    record: &log::Record,
) -> std::io::Result<()> {
    if record.level() == Level::Info {
        write!(write, "{}", &record.args())
    } else {
        Ok(())
    }
}

#[inline]
pub fn file_logger<S: Into<String>, P: Into<PathBuf>>(
    filename: S,
    path: P,
    echo: bool,
) -> Result<Box<Logger>, anyhow::Error> {
    let mut builder = Logger::try_with_str("info")?
        .log_to_file(
            FileSpec::default()
                .basename(filename)
                .directory(path)
                .suppress_timestamp()
                .suffix("txt"),
        )
        .format_for_files(default_format_index)
        .write_mode(WriteMode::Direct)
        .print_message();

    if echo {
        builder = builder.duplicate_to_stdout(flexi_logger::Duplicate::Info);
    }

    Ok(Box::new(builder))
}

macro_rules! impl_message_solver_builder {
    ($p:tt, $t:ty) => {
        impl_message_solver!($p, $t);

        // threads, upper, no poll, no first
        impl MessageSolverBuilder<$p, $t, Yes, Yes, No, No> {
            #[inline]
            pub fn execute(self) -> Result<(), anyhow::Error> {
                let logger = file_logger(self.filename, self.path, self.echo)?;
                logger.start()?;

                let (sx, rx) = mpsc::channel();

                for i in 0..self.threads {
                    let sender: Sender<Permutation<$p>> = sx.clone();
                    thread::spawn(move || {
                        for n in (i as $t..self.upper_bound).step_by(self.threads) {
                            if let Some(sol) = Permutation::<$p>::kth(n + self.start).check_n_s() {
                                match sender.send(sol) {
                                    Ok(_) => {}
                                    Err(_) => {}
                                }
                            }
                        }
                        return;
                    });
                }

                drop(sx);

                let mut recv_iter = rx.iter();
                for _ in 0..self.n {
                    match recv_iter.next() {
                        Some(idxs) => {
                            if self.gen_d {
                                let data = idxs
                                    .generate_d()
                                    .into_iter()
                                    .collect::<BTreeSet<Permutation<$p>>>();

                                for i in data {
                                    log::info!("{}", i.index());
                                    if self.echo {
                                        println!("{}", idxs)
                                    }
                                }
                            } else {
                                log::info!("{}", idxs.index());
                                if self.echo {
                                    println!("{}", idxs)
                                }
                            }
                        }
                        None => break,
                    }
                }

                Ok(())
            }
        }

        // threads, upper, poll, first
        impl MessageSolverBuilder<$p, $t, Yes, Yes, Yes, Yes>
        where
            [(); $p::ELEMENTS]:,
        {
            #[inline]
            pub fn execute<S: Into<String>, P: Into<PathBuf>>(
                self,
                filename: S,
                path: P,
            ) -> Result<(), anyhow::Error> {
                let logger = file_logger(filename, path, self.echo)?;
                logger.start()?;

                let f = Arc::new(AtomicBool::new(false));
                let (sx, rx) = mpsc::channel();

                for i in 0..self.threads {
                    let sender: Sender<Permutation<$p>> = sx.clone();
                    let found = f.clone();
                    thread::spawn(move || {
                        for (count, n) in (i as $t..self.upper_bound)
                            .step_by(self.threads)
                            .enumerate()
                        {
                            if let Some(sol) = Permutation::<$p>::kth(n + self.start).check_n_s() {
                                found.store(self.find_first, Relaxed);
                                match sender.send(sol) {
                                    Ok(_) => return,
                                    Err(_) => {}
                                };
                            } else if count % self.polling_rate == 0 && found.load(Relaxed) {
                                return;
                            };
                        }
                        return;
                    });
                }

                drop(sx);

                match rx.recv() {
                    Ok(idxs) => {
                        if self.gen_d {
                            let data = idxs
                                .generate_d()
                                .into_iter()
                                .collect::<BTreeSet<Permutation<$p>>>();

                            for i in data {
                                log::info!("{}", i.index());
                                if self.echo {
                                    println!("{}", idxs)
                                }
                            }
                        } else {
                            log::info!("{}", idxs.index());
                            if self.echo {
                                println!("{}", idxs)
                            }
                        }

                        Ok(())
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

    use crate::{IndexConst, MessageSolver, O3};

    #[test]
    fn test_builder() -> Result<(), anyhow::Error> {
        MessageSolver::<O3>::default_build()
            .threads(16)
            .upper_bound(O3::MAX_INDEX)
            .n(8)
            .echo(true)
            .output_dir("examples/collected/orderfour/")
            .filename("TestMPSC")
            .execute()?;

        Ok(())
    }
}
