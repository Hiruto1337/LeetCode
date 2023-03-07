struct MinStack {
    head: Option<Box<MinStack>>,
    
}

impl MinStack {
    fn new() -> Self {

    }

    fn push(&self, val: i32) {}

    fn pop(&self) {}

    fn top(&self) -> i32 {}

    fn get_min(&self) -> i32 {}
}

fn main() {}

/*
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
