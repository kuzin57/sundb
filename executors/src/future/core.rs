use anyhow::Result;
use std::sync::{Arc, Condvar, Mutex};

pub trait Future<T> {
    fn consume(&self) -> Option<T>;
}

pub trait Promise<T> {
    fn produce(&mut self, value: T) -> Result<()>;
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
        match self.state.lock() {
            Ok(mut state) => {
                while !state.ready {
                    state = if let Ok(state) = state.wait_cond.clone().wait(state) {
                        state
                    } else {
                        return None;
                    };
                }

                state.value.clone()
            }
            Err(_) => None,
        }
    }
}

impl<T> Promise<T> for SimplePromise<T> {
    fn produce(&mut self, value: T) -> Result<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|e| anyhow::anyhow!("Poisoned mutex: {:?}", e))?;
        state.value = Some(value);
        state.ready = true;
        state.wait_cond.notify_all();
        Ok(())
    }
}
