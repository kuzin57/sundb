use std::{
    cell::RefCell,
    sync::mpsc,
    thread::{self, JoinHandle},
};

use crate::scheduler::core::{RunnableWrapper, Scheduler, SchedulerAdapter};
use crate::scheduler::easy::{EasyScheduler, EasySchedulerAdapter};
use std::rc::Rc;

thread_local! {
    static SCHEDULER: RefCell<Option<Rc<dyn SchedulerAdapter>>> = RefCell::new(None);
}

pub fn init() -> JoinHandle<()> {
    let (sender, receiver) = mpsc::channel();
    let scheduler_adapter: Rc<dyn SchedulerAdapter> = Rc::new(EasySchedulerAdapter::new(sender));

    SCHEDULER.with_borrow_mut(|s| {
        *s = Some(Rc::clone(&scheduler_adapter));
    });

    thread::spawn(move || {
        let mut scheduler: Box<dyn Scheduler> = Box::new(EasyScheduler::new(3, receiver));
        scheduler.run();
    })
}

pub fn go<F>(f: F)
where
    F: FnOnce() + Send + Sync + 'static,
{
    SCHEDULER.with_borrow_mut(|s| {
        let scheduler = if let Some(scheduler) = s.as_mut() {
            scheduler
        } else {
            println!("can not schedule fiber, scheduler is not initialized");
            return;
        };

        let scheduler = if let Some(scheduler) = Rc::get_mut(scheduler) {
            scheduler
        } else {
            println!("can not schedule fiber, scheduler is not initialized");
            return;
        };

        scheduler.schedule(Box::new(RunnableWrapper { f: Some(f) }));
    });
}

pub fn stop() {
    SCHEDULER.with_borrow_mut(|s| {
        let scheduler = if let Some(scheduler) = s.as_mut() {
            scheduler
        } else {
            println!("can not stop scheduler, it is not initialized");
            return;
        };

        if let Some(scheduler) = Rc::get_mut(scheduler) {
            scheduler.stop();
        } else {
            println!("can not stop scheduler, it is not initialized");
        }
    });
}
