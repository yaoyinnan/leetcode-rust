// 2021.12.23
// 剑指 Offer 53 - I. 在排序数组中查找数字 I
// 统计一个数字在排序数组中出现的次数。
// 难度：简单

// 示例 1:
// 输入: nums = [5,7,7,8,8,10], target = 8
// 输出: 2

// 示例 2:
// 输入: nums = [5,7,7,8,8,10], target = 6
// 输出: 0

struct Solution {}

impl Solution {
    fn new() -> Solution {
        Solution {}
    }

    // 直接遍历
    // pub fn search(self, nums: Vec<i32>, target: i32) -> i32 {
    //     let mut count = 0;
    //     for num in nums.iter() {
    //         if *num == target {
    //             count += 1;
    //         }
    //     }
    //     count
    // }

    // 二分法
    // 注意在rust中数组索引只能是usize，而二分查找过程中left和right可能会是负数，此时赋值会产生问题。针对此问题，需要制定好数据类型，并适当做一些类型转换。
    pub fn search(self, nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0{
            return 0;
        }
        let binary_search = |t|{
            let mut left: i32 = 0;
            let mut right: i32 = (nums.len() - 1) as i32;
            while left <= right{
                let m: usize = ((right + left) / 2) as usize;
                if nums[m] <= t {
                    left = m as i32 + 1;
                } else {
                    right = m as i32 - 1;
                }
            }
            left
        };
        (binary_search(target) - binary_search(target - 1)) as i32
    }
}

fn main() {
    let nums = vec![];
    let target = 1;
    let s = Solution::new();
    let repeat_num = s.search(nums, target);
    println!("{}", repeat_num);
}
