use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, AtomicI32, Ordering},
        Arc, Condvar, Mutex,
    },
};

pub trait Sender<T> {
    fn send(&self, item: T);
}

pub trait Closer<T> {
    fn close(&self) -> Result<(), ErrAlreadyClosed>;
}

pub trait Receiver<T> {
    fn recv(&self) -> Option<T>;
}

pub struct MPMCQueue<T: Send + Sync> {
    buffer: Arc<Mutex<VecDeque<T>>>,
    waiters: Arc<AtomicI32>,
    closed: Arc<AtomicBool>,
    cond: Arc<Condvar>,
}

impl<T: Send + Sync> MPMCQueue<T> {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(VecDeque::new())),
            closed: Arc::new(AtomicBool::new(false)),
            waiters: Arc::new(AtomicI32::new(0)),
            cond: Arc::new(Condvar::new()),
        }
    }
}

impl<T: Send + Sync> Default for MPMCQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Send + Sync> Sender<T> for MPMCQueue<T> {
    fn send(&self, item: T) {
        if self.closed.load(Ordering::Acquire) {
            return;
        }

        let mut buffer = self.buffer.lock().unwrap();
        buffer.push_back(item);

        if self.waiters.load(Ordering::Relaxed) > 0 {
            self.waiters.store(0, Ordering::Relaxed);
            self.cond.notify_all();
        }
    }
}

impl<T: Send + Sync> Receiver<T> for MPMCQueue<T> {
    fn recv(&self) -> Option<T> {
        loop {
            let mut buffer = self.buffer.lock().unwrap();
            if let Some(item) = buffer.pop_front() {
                return Some(item);
            }

            if self.closed.load(Ordering::Relaxed) {
                return None;
            }

            self.waiters.fetch_add(1, Ordering::Relaxed);
            let _unused = self.cond.wait(buffer).unwrap();
        }
    }
}

impl<T: Send + Sync> Closer<T> for MPMCQueue<T> {
    fn close(&self) -> Result<(), ErrAlreadyClosed> {
        if self.closed.swap(true, Ordering::Release) {
            return Err(ErrAlreadyClosed {});
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ErrAlreadyClosed {}
