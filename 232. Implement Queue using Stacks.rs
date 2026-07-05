#[derive(Default)]
struct MyQueue {
    input: Vec<i32>,
    output: Vec<i32>,
}


impl MyQueue {
    fn new() -> Self {
        Self { ..Default::default() }    
    }
    
    fn push(&mut self, x: i32) {
        self.input.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        if let Some(x) = self.output.pop() {
            return x;
        }

        while let Some(x) = self.input.pop() {
            self.output.push(x);
        }

        self.pop()
    }
    
    fn peek(&mut self) -> i32 {
        if let Some(x) = self.output.last() {
            return *x;
        }

        while let Some(x) = self.input.pop() {
            self.output.push(x);
        }

        self.peek()
    }
    
    fn empty(&self) -> bool {
        self.input.is_empty() && self.output.is_empty()
    }
}
