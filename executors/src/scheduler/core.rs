use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use crate::sync::queue::{Closer, MPMCQueue, Receiver, Sender};

pub trait Runnable: Send + Sync {
    fn run(&mut self) -> Result<(), RunnableError>;
}

pub struct RunnableError {
    pub message: String,
}

impl Debug for RunnableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RunnableError: {}", self.message)
    }
}

pub struct RunnableWrapper<F>
where
    F: FnOnce() + Send + Sync + 'static,
{
    pub f: Option<F>,
}

impl<F> Runnable for RunnableWrapper<F>
where
    F: FnOnce() + Send + Sync + 'static,
{
    fn run(&mut self) -> Result<(), RunnableError> {
        self.f.take().ok_or(RunnableError {
            message: "RunnableWrapper is already run".to_string(),
        })?();

        Ok(())
    }
}
pub trait Scheduler {
    fn schedule(&mut self, runnable: Box<dyn Runnable>);
    fn stop(&mut self);
}

pub struct EasyScheduler {
    join_handles: Vec<JoinHandle<()>>,
    sender: Arc<dyn Sender<Box<dyn Runnable>>>,
    closer: Arc<dyn Closer<Box<dyn Runnable>>>,
}

impl EasyScheduler {
    fn run_worker(queue: Arc<dyn Receiver<Box<dyn Runnable>>>) {
        loop {
            let opt_runnable = { queue.recv() };

            if let Some(mut runnable) = opt_runnable {
                let result = runnable.run();
                if result.is_err() {
                    println!("some shit happened: {:?}", result.err().unwrap());
                }
            } else {
                println!("Queue is empty {:?}", thread::current().id());
                break;
            }
        }
    }

    pub fn new(workers_count: usize) -> Self {
        let queue = Arc::new(MPMCQueue::<Box<dyn Runnable>>::new());

        let join_handles = (0..workers_count)
            .map(|_| {
                let local_queue = queue.clone();

                thread::spawn(move || {
                    Self::run_worker(local_queue.clone());
                })
            })
            .collect();

        Self {
            join_handles,
            sender: queue.clone(),
            closer: queue.clone(),
        }
    }
}

impl Scheduler for EasyScheduler {
    fn schedule(&mut self, runnable: Box<dyn Runnable>) {
        let _unused = if let Ok(result) = self.sender.send(runnable) {
            result
        } else {
            println!("something went wrong, scheduler is poisoned");
            return;
        };
    }

    fn stop(&mut self) {
        println!("Stopping scheduler");
        let _unused = if let Ok(result) = self.closer.close() {
            result
        } else {
            println!("Scheduler already closed");
            return;
        };

        while let Some(handle) = self.join_handles.pop() {
            if let Err(e) = handle.join() {
                println!("panic handeled in worker: {:?}", e);
                continue;
            } else {
                println!("worker finished");
                continue;
            };
        }
    }
}
