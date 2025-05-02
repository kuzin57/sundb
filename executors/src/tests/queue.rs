pub use std::{sync::Arc, thread};

pub use crate::sync::queue::Closer;
pub use crate::sync::queue::{MPMCQueue, Receiver, Sender};

#[test]
fn test_just_works() {
    let queue = Arc::new(MPMCQueue::new());

    queue.send(1).unwrap();
    queue.send(2).unwrap();
    queue.send(3).unwrap();

    let result = queue.close();
    assert!(result.is_ok());

    let result_close = queue.close();
    assert!(result_close.is_err());

    assert_eq!(queue.recv(), Some(1));
    assert_eq!(queue.recv(), Some(2));
    assert_eq!(queue.recv(), Some(3));
    assert_eq!(queue.recv(), None);
}
