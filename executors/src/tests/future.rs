pub use std::{
    sync::{Arc, Mutex},
    thread,
};

pub use crate::future::core::{BlockingFuture, Future, Promise, SharedState, SimplePromise};

#[test]
fn test_just_works() {
    let state = Arc::new(Mutex::new(SharedState::<i32>::new()));
    let future = BlockingFuture::new(Arc::clone(&state));
    let mut promise = SimplePromise::new(Arc::clone(&state));

    promise.produce(1);

    assert_eq!(future.consume(), Some(1));
}

#[test]
fn test_concurrent() {
    let state = Arc::new(Mutex::new(SharedState::<i32>::new()));
    let future = BlockingFuture::new(Arc::clone(&state));
    let mut promise = SimplePromise::new(Arc::clone(&state));

    let handle = thread::spawn(move || {
        promise.produce(1);
    });

    handle.join().unwrap();

    assert_eq!(future.consume(), Some(1));
}
