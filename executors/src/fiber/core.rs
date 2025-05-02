use std::cell::RefCell;

use crate::scheduler::core::{EasyScheduler, RunnableWrapper, Scheduler};
use std::rc::Rc;

thread_local! {
    static SCHEDULER: RefCell<Option<Rc<dyn Scheduler>>> = RefCell::new(None);
}

pub fn init() {
    let scheduler: Rc<dyn Scheduler> = Rc::new(EasyScheduler::new(3));

    SCHEDULER.with_borrow_mut(|s| {
        *s = Some(Rc::clone(&scheduler));
    });
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
