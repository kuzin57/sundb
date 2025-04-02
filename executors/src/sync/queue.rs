use std::collections::VecDeque;

pub trait Queue<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn close(&mut self);
}

pub struct SimpleQueue<T> {
    buf: VecDeque<T>,
    closed: bool,
}

impl<T> SimpleQueue<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: VecDeque::with_capacity(capacity),
            closed: false,
        }
    }
}

impl<T> Queue<T> for SimpleQueue<T> {
    fn push(&mut self, item: T) {
        if self.closed {
            return;
        }

        self.buf.push_back(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.buf.pop_front()
    }

    fn close(&mut self) {
        self.closed = true;
    }
}

unsafe impl<T> Send for SimpleQueue<T> {}

unsafe impl<T> Sync for SimpleQueue<T> {}
