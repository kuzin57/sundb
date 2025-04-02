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
    F: FnOnce() + Send + 'static,
{
    SCHEDULER.with_borrow_mut(|s| {
        let scheduler = s.as_mut().unwrap();

        Rc::get_mut(scheduler)
            .unwrap()
            .schedule(Box::new(RunnableWrapper { f: Some(f) }));
    });
}

pub fn stop() {
    SCHEDULER.with_borrow_mut(|s| {
        let scheduler = s.as_mut().unwrap();
        Rc::get_mut(scheduler).unwrap().stop();
    });
}
