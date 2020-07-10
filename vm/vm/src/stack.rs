pub struct VMStack {
    internal_stack: Vec<i32>,
    capacity: usize,
    top: usize,
}

impl VMStack {
    pub fn new(stack_size: usize) -> VMStack {
        VMStack {
            capacity: stack_size.clone(),
            top: 0,
            internal_stack: Vec::with_capacity(stack_size),
        }
    }

    pub fn push(&mut self, data: i32) {
        if self.top == self.capacity {
            panic!("Stack overflow Capacity {} , Size {} ", self.capacity, self.top)
        }
        self.internal_stack.push(data);
        self.top += 1;
    }

    pub fn pop(&mut self) -> i32 {
        if self.top == 0 {
            panic!("Stack underflow")
        }
        self.top -= 1;
        self.internal_stack.pop().unwrap()
    }

    pub fn peek(&self) -> i32 {
        *self.internal_stack.last().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use crate::stack::VMStack;

    #[test]
    fn test_stack() {
        let mut stack = VMStack::new(5);
        stack.push(12);
        assert_eq!(stack.capacity, 5);
        assert_eq!(stack.top, 1);
        assert_eq!(stack.internal_stack.to_vec(), [12]);
        stack.pop();
        assert_eq!(stack.capacity, 5);
        assert_eq!(stack.top, 0);
    }

    #[test]
    fn test_vm_stack_push_pop() {
        let mut stack = VMStack::new(1);
        stack.push(10);

        assert_eq!(10, stack.peek());
        assert_eq!(10, stack.pop());
    }
}
