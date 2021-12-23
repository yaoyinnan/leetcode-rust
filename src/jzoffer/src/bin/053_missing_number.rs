// 2021.12.23
// 剑指 Offer 53 - II. 0～n-1中缺失的数字
// 一个长度为n-1的递增排序数组中的所有数字都是唯一的，并且每个数字都在范围0～n-1之内。在范围0～n-1内的n个数字中有且只有一个数字不在该数组中，请找出这个数字。
// 难度：简单

// 示例 1:
// 输入: [0,1,3]
// 输出: 2

// 示例 2:
// 输入: [0,1,2,3,4,5,6,7,9]
// 输出: 8

struct Solution {}

impl Solution {
    fn new() -> Solution {
        Solution {}
    }

    pub fn missing_number(self, nums: Vec<i32>) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() - 1) as i32;
        while left <= right {
            let m: usize = ((left + right) / 2) as usize;
            if nums[m] <= m as i32 {
                left = m as i32 + 1;
            } else {
                right = m as i32 - 1;
            }
        }
        left
    }
}

fn main() {
    let nums = vec![0,1,2,3,5,6,7,8,9];
    let s = Solution::new();
    let repeat_num = s.missing_number(nums);
    println!("{}", repeat_num);
}
