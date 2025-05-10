pub use std::{
    thread::{self, sleep},
    time::{Duration, Instant},
};

pub use crate::fiber::core::{init, spawn, stop};

#[test]
pub fn test_just_works() {
    let join_handle = init();

    let start = Instant::now();

    spawn(|| {
        sleep(Duration::from_secs(1));
        println!("Hello, world {:?}", thread::current().id());
    });

    spawn(|| {
        sleep(Duration::from_secs(1));
        println!("Hello, world {:?}", thread::current().id());
    });

    spawn(|| {
        sleep(Duration::from_secs(1));
        println!("Hello, world {:?}", thread::current().id());
    });

    stop();
    join_handle.join().unwrap();

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
    assert!(duration < Duration::from_millis(1100));
}
