use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

use crate::scheduler::core::{Runnable, Scheduler, SchedulerAdapter};

struct WorkerAdapter {
    sender: Sender<Option<Box<dyn Runnable>>>,
    tasks_count: usize,
}

impl Ord for WorkerAdapter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tasks_count.cmp(&other.tasks_count)
    }
}

impl PartialOrd for WorkerAdapter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for WorkerAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.tasks_count == other.tasks_count
    }
}

impl Eq for WorkerAdapter {}

impl WorkerAdapter {
    fn new(sender: Sender<Option<Box<dyn Runnable>>>) -> Self {
        Self {
            sender,
            tasks_count: 0,
        }
    }

    pub fn send(&mut self, runnable: Box<dyn Runnable>) {
        self.sender.send(Some(runnable)).unwrap();
        self.tasks_count += 1;
    }

    pub fn stop(&mut self) {
        self.sender.send(None).unwrap();
    }
}

pub struct Worker {
    receiver: Receiver<Option<Box<dyn Runnable>>>,
}

impl Worker {
    fn new(receiver: Receiver<Option<Box<dyn Runnable>>>) -> Self {
        Self { receiver }
    }

    fn run(&mut self) {
        while let Ok(runnable) = self.receiver.recv() {
            if let Some(mut runnable) = runnable {
                if let Err(e) = runnable.run() {
                    println!("some shit happened: {:?}", e);
                }
            } else {
                break;
            }
        }

        println!("worker finished: {:?}", thread::current().id());
    }
}

pub struct EasySchedulerAdapter {
    sender: Sender<Option<Box<dyn Runnable>>>,
}

impl EasySchedulerAdapter {
    pub fn new(sender: Sender<Option<Box<dyn Runnable>>>) -> Self {
        Self { sender }
    }
}

impl SchedulerAdapter for EasySchedulerAdapter {
    fn schedule(&mut self, runnable: Box<dyn Runnable>) {
        if let Err(e) = self.sender.send(Some(runnable)) {
            println!("something went wrong, scheduler is poisoned: {:?}", e);
        }
    }

    fn stop(&mut self) {
        if let Err(e) = self.sender.send(None) {
            println!("something went wrong, scheduler is poisoned: {:?}", e);
        }
    }
}

pub struct EasyScheduler {
    receiver: Receiver<Option<Box<dyn Runnable>>>,
    join_handles: Vec<JoinHandle<()>>,
    workers_adapters: BinaryHeap<Reverse<WorkerAdapter>>,
}

impl EasyScheduler {
    pub fn new(workers_count: usize, receiver: Receiver<Option<Box<dyn Runnable>>>) -> Self {
        let mut workers_adapters = BinaryHeap::new();

        let join_handles = (0..workers_count)
            .map(|_| {
                let (sender, receiver) = mpsc::channel();
                let worker_adapter = WorkerAdapter::new(sender);
                workers_adapters.push(Reverse(worker_adapter));

                thread::spawn(move || {
                    let mut worker = Worker::new(receiver);
                    worker.run();
                })
            })
            .collect();

        Self {
            join_handles,
            receiver,
            workers_adapters,
        }
    }
}

impl Scheduler for EasyScheduler {
    fn run(&mut self) {
        while let Ok(runnable) = self.receiver.recv() {
            if let Some(runnable) = runnable {
                if let Some(mut worker_adapter) = self.workers_adapters.peek_mut() {
                    worker_adapter.0.send(runnable);
                } else {
                    println!("No workers available");
                    break;
                }
            } else {
                break;
            }
        }

        for mut worker_adapter in self.workers_adapters.drain() {
            worker_adapter.0.stop();
        }

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
