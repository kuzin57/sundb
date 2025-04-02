pub use crate::sync::queue::{Queue, SimpleQueue};

#[test]
fn test_just_works() {
    let mut queue: SimpleQueue<i32> = SimpleQueue::new(10);

    queue.push(1);
    queue.push(2);
    queue.push(3);

    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.pop(), Some(3));
    assert_eq!(queue.pop(), None);
}

#[test]
fn test_closed_queue() {
    let mut queue: SimpleQueue<i32> = SimpleQueue::new(10);

    queue.push(1);
    queue.push(2);
    queue.push(3);

    queue.close();

    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.pop(), Some(3));
    assert_eq!(queue.pop(), None);

    queue.push(4);
    assert_eq!(queue.pop(), None);
}
