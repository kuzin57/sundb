use std::fmt;
use std::fmt::Debug;

pub trait Runnable: Send + Sync {
    fn run(&mut self) -> Result<(), RunnableError>;
}

pub struct RunnableError {
    pub message: String,
}

impl Debug for RunnableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RunnableError: {}", self.message)
    }
}

pub struct RunnableWrapper<F>
where
    F: FnOnce() + Send + Sync + 'static,
{
    pub f: Option<F>,
}

impl<F> Runnable for RunnableWrapper<F>
where
    F: FnOnce() + Send + Sync + 'static,
{
    fn run(&mut self) -> Result<(), RunnableError> {
        self.f.take().ok_or(RunnableError {
            message: "RunnableWrapper is already run".to_string(),
        })?();

        Ok(())
    }
}
pub trait SchedulerAdapter {
    fn schedule(&mut self, runnable: Box<dyn Runnable>);
    fn stop(&mut self);
}

pub trait Scheduler {
    fn run(&mut self);
}
