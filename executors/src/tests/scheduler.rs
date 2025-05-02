pub use std::sync::mpsc;
pub use std::thread;

pub use chrono::DateTime;

pub use crate::scheduler::core::SchedulerAdapter;
pub use crate::scheduler::core::{Runnable, RunnableError, Scheduler};
pub use crate::scheduler::easy::EasyScheduler;
pub use crate::scheduler::easy::EasySchedulerAdapter;

#[test]
pub fn test_just_works() {
    println!("Starting test");
    let (sender, receiver) = mpsc::channel();

    let join_scheduler = thread::spawn(move || {
        let mut scheduler: Box<dyn Scheduler> = Box::new(EasyScheduler::new(3, receiver));
        scheduler.run();
    });
    let mut scheduler_adapter: Box<dyn SchedulerAdapter> =
        Box::new(EasySchedulerAdapter::new(sender));

    println!("Scheduler created");

    scheduler_adapter.schedule(Box::new(SimpleTask { id: 1 }));
    scheduler_adapter.schedule(Box::new(SimpleTask { id: 2 }));
    scheduler_adapter.schedule(Box::new(SimpleTask { id: 3 }));

    scheduler_adapter.stop();
    join_scheduler.join().unwrap();
}

#[test]
pub fn test_concurrent() {
    static BIG_VECTOR: [i32; 300000] = [1; 300000];
    let (sender, receiver) = mpsc::channel();

    let join_scheduler = thread::spawn(move || {
        let mut scheduler = EasyScheduler::new(3, receiver);
        scheduler.run();
    });
    let mut scheduler_adapter: Box<dyn SchedulerAdapter> =
        Box::new(EasySchedulerAdapter::new(sender));

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

    let start = chrono::Local::now();
    scheduler_adapter.schedule(first_summator);
    scheduler_adapter.schedule(second_summator);
    scheduler_adapter.schedule(third_summator);
    let duration = chrono::Local::now().signed_duration_since(start);
    println!("Time taken: {:?} ns", duration.num_nanoseconds().unwrap());

    scheduler_adapter.stop();
    join_scheduler.join().unwrap();
}

pub struct SimpleTask {
    id: usize,
}

impl Runnable for SimpleTask {
    fn run(&mut self) -> Result<(), RunnableError> {
        println!("SimpleTask {} is running", self.id);
        Ok(())
    }
}

pub struct Summator<'a> {
    id: usize,
    slice: &'a [i32],
}

impl Runnable for Summator<'_> {
    fn run(&mut self) -> Result<(), RunnableError> {
        let mut sum = 0;
        for i in self.slice {
            sum += i;
        }
        println!("Summator {} is done, sum is {}", self.id, sum);
        Ok(())
    }
}
