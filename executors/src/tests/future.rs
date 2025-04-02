use std::time::Instant;

use crate::{
    future::core::{BlockingFuture, Future, Promise, SharedState, SimplePromise},
    scheduler::core::{EasyScheduler, Runnable, Scheduler},
};

#[test]
fn test_just_works() {
    let state = SharedState::<i32>::new();
    let future = BlockingFuture::new(state);
    let promise = SimplePromise::new(state);

    promise.produce(1);

    assert_eq!(future.consume(), Some(1));
}

#[test]
fn test_concurrent() {
    static BIG_VECTOR: [i32; 300000] = [1; 300000];

    let mut scheduler = EasyScheduler::new(3);

    let first_summator = Box::new(Summator {
        id: 1,
        slice: &BIG_VECTOR[..100000],
    });
    let second_summator = Box::new(Summator {
        id: 2,
        slice: &BIG_VECTOR[100000..200000],
    });
    let third_summator = Box::new(Summator {
        id: 3,
        slice: &BIG_VECTOR[200000..],
    });

    let mut start = Instant::now();
    scheduler.schedule(first_summator);
    scheduler.schedule(second_summator);
    scheduler.schedule(third_summator);
    let mut duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    scheduler.stop();
}

struct SimpleTask {
    id: usize,
}

impl Runnable for SimpleTask {
    fn run(&self) {
        println!("SimpleTask {} is running", self.id);
    }
}

struct Summator<'a> {
    id: usize,
    slice: &'a [i32],
}

impl<'a> Runnable for Summator<'a> {
    fn run(&self) {
        let mut sum = 0;
        for i in self.slice {
            sum += i;
        }
        println!("Summator {} is done, sum is {}", self.id, sum);
    }
}
