use std::sync::{Arc, Condvar, Mutex};

pub trait Future<T> {
    fn consume(&self) -> Option<T>;
}

pub trait Promise<T> {
    fn produce(&mut self, value: T);
}

pub struct SharedState<T> {
    value: Option<T>,
    ready: bool,
    wait_cond: Arc<Condvar>,
}

impl<T> SharedState<T> {
    pub fn new() -> Self {
        Self {
            value: None,
            ready: false,
            wait_cond: Arc::new(Condvar::new()),
        }
    }
}

impl<T> Default for SharedState<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BlockingFuture<T> {
    state: Arc<Mutex<SharedState<T>>>,
}

impl<T> BlockingFuture<T> {
    pub fn new(state: Arc<Mutex<SharedState<T>>>) -> Self {
        Self { state }
    }
}
pub struct SimplePromise<T> {
    state: Arc<Mutex<SharedState<T>>>,
}

impl<T> SimplePromise<T> {
    pub fn new(state: Arc<Mutex<SharedState<T>>>) -> Self {
        Self { state }
    }
}

impl<T: Clone> Future<T> for BlockingFuture<T> {
    fn consume(&self) -> Option<T> {
        let mut state = self.state.lock().unwrap();

        while !state.ready {
            state = state.wait_cond.clone().wait(state).unwrap();
        }

        state.value.clone()
    }
}

impl<T> Promise<T> for SimplePromise<T> {
    fn produce(&mut self, value: T) {
        let mut state = self.state.lock().unwrap();
        state.value = Some(value);
        state.ready = true;
        state.wait_cond.notify_all();
    }
}
