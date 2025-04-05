use std::sync::Arc;
use std::thread::{self, JoinHandle};

use crate::sync::queue::{Closer, MPMCQueue, Receiver, Sender};

pub trait Runnable: Send + Sync {
    fn run(&mut self);
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
    fn run(&mut self) {
        (self.f.take().unwrap())();
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
                runnable.run();
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
        self.sender.send(runnable);
    }

    fn stop(&mut self) {
        println!("Stopping scheduler");
        let result = self.closer.close();
        if let Err(_) = result {
            println!("Scheduler already closed");
            return;
        }

        while let Some(handle) = self.join_handles.pop() {
            handle.join().unwrap();
        }
    }
}
