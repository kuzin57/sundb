use crate::sync::queue::{Queue, SimpleQueue};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub trait Runnable {
    fn run(&self);
}

pub trait Scheduler {
    fn schedule(&mut self, runnable: Box<dyn Runnable>);
    fn stop(&mut self);
}

pub struct EasyScheduler {
    join_handles: Vec<JoinHandle<()>>,
    queue: Arc<Mutex<SimpleQueue<Box<dyn Runnable>>>>,
}

impl EasyScheduler {
    pub fn new(workers_count: usize) -> Self {
        let queue = Arc::new(Mutex::new(SimpleQueue::<Box<dyn Runnable>>::new(
            workers_count,
        )));

        let join_handles = (0..workers_count)
            .map(|_| {
                let local_queue = Arc::clone(&queue);

                thread::spawn(move || {
                    while let Some(runnable) = local_queue.lock().unwrap().pop() {
                        runnable.run();
                    }
                })
            })
            .collect();

        Self {
            join_handles,
            queue,
        }
    }
}

impl Scheduler for EasyScheduler {
    fn schedule(&mut self, runnable: Box<dyn Runnable>) {
        self.queue.lock().unwrap().push(runnable);
    }

    fn stop(&mut self) {
        self.queue.lock().unwrap().close();

        while let Some(handle) = self.join_handles.pop() {
            handle.join().unwrap();
        }
    }
}

unsafe impl Send for EasyScheduler {}

unsafe impl Sync for EasyScheduler {}
