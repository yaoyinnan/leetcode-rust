// 2021.12.24
// 剑指 Offer 50. 第一个只出现一次的字符
// 在字符串 s 中找出第一个只出现一次的字符。如果没有，返回一个单空格。 s 只包含小写字母。
// 难度：简单

// 示例 1:
// 输入：s = "abaccdeff"
// 输出：'b'

// 示例 2:
// 输入：s = ""
// 输出：' '


pub fn first_uniq_char(s: String) -> char {
    use std::collections::HashMap;
    let mut hmap = HashMap::new();
    for c in s.chars() {
        hmap.insert(c, !hmap.contains_key(&c));
    }

    for c in s.chars() {
        if match hmap.get(&c) {
            Some(b) => *b,
            None => false,
        } {
            return c;
        }
    }

    ' '
}

fn main() {
    assert_eq!('b', first_uniq_char(String::from("abaccdeff")));
    assert_eq!(' ', first_uniq_char(String::from("")));
    assert_eq!('a', first_uniq_char(String::from("abc")));
    assert_eq!('l', first_uniq_char(String::from("leetcode")));
}
