#![allow(dead_code)]
struct MyQueue {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue { data: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.data.push(x)
    }

    fn pop(&mut self) -> i32 {
        if self.data.len() == 0 {
            return 0;
        }
        let res = self.data[0];
        self.data.remove(0);
        return res;
    }

    fn peek(&self) -> i32 {
        *self.data.get(0).unwrap_or(&0)
    }

    fn empty(&self) -> bool {
        self.data.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
fn main() {
    let mut obj = MyQueue::new();
    let i = obj.pop();
    println!("{}", i)
}
