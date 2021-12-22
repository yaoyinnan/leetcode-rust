// 2021.12.22
// 剑指 Offer 05. 替换空格
// 请实现一个函数，把字符串 s 中的每个空格替换成"%20"。
// 难度：简单

// 示例 1：
// 输入：s = "We are happy."
// 输出："We%20are%20happy."

struct Solution{

}

impl Solution {
    fn new() -> Solution{
        Solution{}
    }

    pub fn replace_space(&self, s: String) -> String {
        let mut new_str = String::new();

        for c in s.chars(){
            match c{
                ' ' => new_str += "%20",
                _ => new_str += &c.to_string(),
            }
        }

        new_str
    }
}

fn main() {
    let mut str = String::from("Hello world");
    let s = Solution::new();
    println!("{}", str);
    str = s.replace_space(str);
    println!("{}", str);
}
