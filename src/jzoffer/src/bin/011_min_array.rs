// 2021.12.24
// 剑指 Offer 11. 旋转数组的最小数字
// 把一个数组最开始的若干个元素搬到数组的末尾，我们称之为数组的旋转。
// 给你一个可能存在 重复 元素值的数组 numbers ，它原来是一个升序排列的数组，并按上述情形进行了一次旋转。请返回旋转数组的最小元素。例如，数组 [3,4,5,1,2] 为 [1,2,3,4,5] 的一次旋转，该数组的最小值为1。
// 难度：简单

// 示例 1：
// 输入：[3,4,5,1,2]
// 输出：1

// 示例 2：
// 输入：[2,2,2,0,1]
// 输出：0

// 二分法
// 当number[m] > number[j]时，旋转点一定出现在右侧序列。
// 当number[m] < number[j]时，旋转点一定出现在左侧序列。
// 当number[m] = number[j]时，无法判断旋转点位置，依次缩小右边界。

pub fn min_array(numbers: Vec<i32>) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = numbers.len() as i32 - 1;
    while left <= right {
        let m = ((left + right) / 2) as usize;
        if numbers[m] > numbers[right as usize] {
            left = m as i32 + 1;
        } else if numbers[m] < numbers[right as usize] {
            right = m as i32;
        } else {
            right -= 1;
        }
    }
    match numbers.get(left as usize) {
        Some(v) => *v,
        None => -1,
    }
}

fn main() {
    assert_eq!(0, min_array(vec![2, 2, 2, 0, 1]));
    assert_eq!(1, min_array(vec![5, 1, 2, 3, 4]));
    assert_eq!(2, min_array(vec![2, 2, 2, 2, 2]));
    assert_eq!(1, min_array(vec![3, 1]));
    assert_eq!(1, min_array(vec![3, 1, 3, 3]));
    assert_eq!(-1, min_array(vec![]));
}
