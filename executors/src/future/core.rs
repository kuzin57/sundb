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
}

impl<T> SharedState<T> {
    pub fn new() -> Self {
        Self {
            value: None,
            ready: false,
        }
    }
}
pub struct BlockingFuture<T> {
    state: Arc<Mutex<SharedState<T>>>,
    condvar: Arc<Condvar>,
}

impl<T> BlockingFuture<T> {
    pub fn new(state: SharedState<T>) -> Self {
        Self {
            state: Arc::new(Mutex::new(state)),
            condvar: Arc::new(Condvar::new()),
        }
    }
}
pub struct SimplePromise<T> {
    state: Arc<Mutex<SharedState<T>>>,
    condvar: Arc<Condvar>,
}

impl<T> SimplePromise<T> {
    pub fn new(state: SharedState<T>) -> Self {
        Self {
            state: Arc::new(Mutex::new(state)),
            condvar: Arc::new(Condvar::new()),
        }
    }
}

impl<T: Copy + Clone> Future<T> for BlockingFuture<T> {
    fn consume(&self) -> Option<T> {
        let mut state = self.state.lock().unwrap();

        while !state.ready {
            state = self.condvar.wait(state).unwrap();
        }

        state.value.clone()
    }
}

impl<T> Promise<T> for SimplePromise<T> {
    fn produce(&mut self, value: T) {
        let mut state = self.state.lock().unwrap();
        state.value = Some(value);
        state.ready = true;
        self.condvar.notify_all();
    }
}
