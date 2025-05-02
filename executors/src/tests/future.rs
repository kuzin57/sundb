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

    let result = promise.produce(1);
    assert!(result.is_ok());

    assert_eq!(future.consume(), Some(1));
}

#[test]
fn test_concurrent() {
    let state = Arc::new(Mutex::new(SharedState::<i32>::new()));
    let future = BlockingFuture::new(Arc::clone(&state));
    let mut promise = SimplePromise::new(Arc::clone(&state));

    let handle = thread::spawn(move || {
        let result = promise.produce(1);
        assert!(result.is_ok());
    });

    handle.join().unwrap();

    assert_eq!(future.consume(), Some(1));
}

#[test]
fn test_concurrent_future() {
    let state = Arc::new(Mutex::new(SharedState::<i32>::new()));
    let future = BlockingFuture::new(Arc::clone(&state));
    let mut promise = SimplePromise::new(Arc::clone(&state));

    let handle = thread::spawn(move || {
        assert_eq!(future.consume(), Some(1));
    });

    let result = promise.produce(1);
    assert!(result.is_ok());

    handle.join().unwrap();
}
