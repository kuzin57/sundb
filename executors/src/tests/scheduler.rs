pub use chrono::DateTime;

pub use crate::scheduler::core::{EasyScheduler, Runnable, RunnableError, Scheduler};

#[test]
pub fn test_just_works() {
    println!("Starting test");
    let mut scheduler = EasyScheduler::new(3);

    println!("Scheduler created");

    scheduler.schedule(Box::new(SimpleTask { id: 1 }));
    scheduler.schedule(Box::new(SimpleTask { id: 2 }));
    scheduler.schedule(Box::new(SimpleTask { id: 3 }));

    scheduler.stop();
}

#[test]
pub fn test_concurrent() {
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

    let start = chrono::Local::now();
    scheduler.schedule(first_summator);
    scheduler.schedule(second_summator);
    scheduler.schedule(third_summator);
    let duration = chrono::Local::now().signed_duration_since(start);
    println!("Time taken: {:?} ns", duration.num_nanoseconds().unwrap());

    scheduler.stop();
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
