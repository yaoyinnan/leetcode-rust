// 2021.12.21
// 剑指 Offer 30. 包含min函数的栈
// 定义栈的数据结构，请在该类型中实现一个能够得到栈的最小元素的 min 函数在该栈中，调用 min、push 及 pop 的时间复杂度都是 O(1)。
// 难度：简单

// 示例:
// MinStack minStack = new MinStack();
// minStack.push(-2);
// minStack.push(0);
// minStack.push(-3);
// minStack.min();   --> 返回 -3.
// minStack.pop();
// minStack.top();      --> 返回 0.
// minStack.min();   --> 返回 -2.

struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            stack: vec![],
            min_stack: vec![],
        }
    }
    fn push(&mut self, x: i32) {
        self.stack.push(x);

        let min = match self.min_stack.last(){
            Some(v) if v > &x => x,
            Some(v) => *v,
            None => x
        };

        self.min_stack.push(min);
    }
    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    fn min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.min();
 */

fn main() {
    let mut min_stack = MinStack::new();
    min_stack.push(2);
    min_stack.push(0);
    min_stack.push(3);
    min_stack.push(0);
    println!("{}", min_stack.min());
    min_stack.pop();
    println!("{}", min_stack.top());
    println!("{}", min_stack.min());
    min_stack.pop();
    println!("{}", min_stack.min());
    min_stack.pop();
    println!("{}", min_stack.min());
}
