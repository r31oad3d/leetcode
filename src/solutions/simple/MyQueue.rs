use std::collections::VecDeque;
struct MyQueue {
    elem: VecDeque<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue{elem: VecDeque::<i32>::new()}
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.elem.push_back(x)
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        self.elem.pop_front().unwrap()
    }

    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        *self.elem.get(0).unwrap()
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.elem.is_empty()
    }
}

