// 2021.12.20
// 剑指 Offer 09. 用两个栈实现队列
// 用两个栈实现一个队列。队列的声明如下，请实现它的两个函数 appendTail 和 deleteHead ，分别完成在队列尾部插入整数和在队列头部删除整数的功能。(若队列中没有元素，deleteHead 操作返回 -1 )
// 难度：简单

// 示例 1：
// 输入：
// ["CQueue","appendTail","deleteHead","deleteHead"]
// [[],[3],[],[]]
// 输出：[null,null,3,-1]

// 示例 2：
// 输入：
// ["CQueue","deleteHead","appendTail","appendTail","deleteHead","deleteHead"]
// [[],[],[5],[2],[],[]]
// 输出：[null,-1,null,null,5,2]


pub struct CQueue {
    input: Vec<i32>,
    output: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    pub fn new() -> Self {
        CQueue {
            input: Vec::new(),
            output: Vec::new(),
        }
    }
    pub fn append_tail(&mut self, value: i32) {
        self.input.push(value);
    }
    pub fn delete_head(&mut self) -> i32 {
        if let Some(val) = self.output.pop() {
            return val;
        }
        while let Some(val) = self.input.pop() {
            self.output.push(val);
        }
        match self.output.pop() {
            Some(val) => val,
            None => -1,
        }
    }
}

/**
 * Your CQueue object will be instantiated and called as such:
 * let obj = CQueue::new();
 * obj.append_tail(value);
 * let ret_2: i32 = obj.delete_head();
 */

fn main() {
    let mut c = CQueue::new();
    c.append_tail(3);
    let result = c.delete_head();
    println!("{}", result);
    let result = c.delete_head();
    println!("{}", result);
}
