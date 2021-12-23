// 2021.12.23
// 剑指 Offer 03. 数组中重复的数字
// 找出数组中重复的数字。
// 在一个长度为 n 的数组 nums 里的所有数字都在 0～n-1 的范围内。数组中某些数字是重复的，但不知道有几个数字重复了，也不知道每个数字重复了几次。请找出数组中任意一个重复的数字。
// 难度：简单

// 示例 1：
// 输入：
// [2, 3, 1, 0, 2, 5, 3]
// 输出：2 或 3 

// 方法二中使用了HashSet
// use std::collections::HashSet;

struct Solution{

}

impl Solution {
    fn new() -> Solution{
        Solution{}
    }

    // 循环笨方法
    // pub fn find_repeat_number(self, mut nums: Vec<i32>) -> i32 {
    //     nums.sort();
    //     let mut repeat_num = &nums[0];
    //     let first: usize = 1;
    //     let end: usize = nums.len();
    //     for i in first..end{
    //         if *repeat_num == nums[i]{
    //             break
    //         } else {
    //             repeat_num = &nums[i];
    //         }
    //     }
    //     *repeat_num
    // }

    // // HashSet
    // pub fn find_repeat_number(self, nums: Vec<i32>) -> i32 {
    //     let mut hashset = HashSet::new();
    //     for num in nums.iter(){
    //         if hashset.contains(num){
    //             return *num;
    //         } else {
    //             hashset.insert(num);
    //         }
    //     }
    //     -1
    // }

    // 借助题目“长度为n的数组里，所有元素都在0 ~ n-1范围内”的特性，以数组元素值为索引，判断元素值与索引元素值是否相等，如果相等则证明重复，否则交换。
    // TODO 存在问题
    pub fn find_repeat_number(self, mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            println!("{:?}", nums);
            let index = nums[i] as usize;
            if nums[i] == nums[index] && i != index {
                return nums[i];
            }
            else {
                let t = nums[i];
                nums[i] = nums[index];
                nums[index] = t;
            }
        }
        -1
   }
}

fn main(){
    let nums = vec![3, 4, 2, 0, 0, 1];
    let s = Solution::new();
    let repeat_num = s.find_repeat_number(nums);
    println!("{}", repeat_num);
}