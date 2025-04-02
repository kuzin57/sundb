use crate::sync::queue::{Queue, SimpleQueue};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub trait Runnable {
    fn run(&mut self);
}

pub struct RunnableWrapper<F>
where
    F: FnOnce() + Send + 'static,
{
    pub f: Option<F>,
}

impl<F> Runnable for RunnableWrapper<F>
where
    F: FnOnce() + Send + 'static,
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

                thread::spawn(move || loop {
                    let opt_runnable = { local_queue.lock().unwrap().pop() };

                    if let Some(mut runnable) = opt_runnable {
                        runnable.run();
                    } else {
                        println!("Queue is empty {:?}", thread::current().id());
                        break;
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
