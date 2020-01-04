struct MyStack {
    elem: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack{elem: Vec::<i32>::new()}
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.elem.push(x)
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        self.elem.pop().unwrap()
    }

    /** Get the top element. */
    fn top(&mut self) -> i32 {
        *self.elem.last().unwrap()
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.elem.is_empty()
    }
}

