// 2021.12.24
// 剑指 Offer 04. 二维数组中的查找
// 在一个 n * m 的二维数组中，每一行都按照从左到右递增的顺序排序，每一列都按照从上到下递增的顺序排序。请完成一个高效的函数，输入这样的一个二维数组和一个整数，判断数组中是否含有该整数。
// 难度：中等

// 现有矩阵 matrix 如下：

// [
//   [1,   4,  7, 11, 15],
//   [2,   5,  8, 12, 19],
//   [3,   6,  9, 16, 22],
//   [10, 13, 14, 17, 24],
//   [18, 21, 23, 26, 30]
// ]
// 给定 target = 5，返回 true。

// 给定 target = 20，返回 false。

struct Solution {}

impl Solution {
    fn new() -> Solution {
        Solution {}
    }

    // 先二分查找搜索边界，然后遍历边界内的。
    // pub fn find_number_in2_d_array(self, matrix: Vec<Vec<i32>>, target: i32) -> bool {
    //     let n = matrix.len();
    //     let m = match matrix.get(0) {
    //         Some(v) => v.len(),
    //         None => 0,
    //     };

    //     let mut left_n: i32 = 0;
    //     let mut right_n: i32 = m as i32 - 1;

    //     while left_n <= right_n {
    //         let mid: usize = ((left_n + right_n) / 2) as usize;
    //         let val = match matrix[0].get(mid){
    //             Some(v) => v,
    //             None => return false,
    //         };
    //         if *val < target {
    //             left_n = mid as i32 + 1;
    //         } else if *val == target {
    //             return true;
    //         } else {
    //             right_n = mid as i32 - 1;
    //         }
    //     }

    //     let mut left_m: i32 = 0;
    //     let mut right_m: i32 = n as i32 - 1;

    //     while left_m <= right_m {
    //         let mid: usize = ((left_m + right_m) / 2) as usize;
    //         let val = match matrix[mid].get(0){
    //             Some(v) => v,
    //             None => return false,
    //         };
    //         if *val < target {
    //             left_m = mid as i32 + 1;
    //         } else if *val == target {
    //             return true;
    //         } else {
    //             right_m = mid as i32 - 1;
    //         }
    //     }

    //     for i in 0..left_m as usize {
    //         for j in 0..left_n as usize {
    //             if matrix[i][j] == target {
    //                 return true;
    //             }
    //         }
    //     }

    //     false
    // }

    // 二叉搜索树查找
    pub fn find_number_in2_d_array(self, matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut i: i32 = matrix.len() as i32 - 1;
        let mut j: i32 = 0;
        let m: i32 = match matrix.get(0) {
            Some(v) => v.len() as i32,
            None => 0,
        };
        while i >= 0 && j < m {
            let val = matrix[i as usize][j as usize];
            if val == target {
                return true;
            } else if val < target {
                j += 1;
            } else {
                i -= 1;
            }
        }
        false
    }
}

fn main() {
    // let nums = vec![];
    // let nums = vec![vec![1, 1]];
    // let nums = vec![
    //     vec![1],
    //     vec![2],
    //     vec![3],
    //     vec![10],
    //     vec![18],
    //   ];
    let nums = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];
    let target = 2;
    let s = Solution::new();
    let flag = s.find_number_in2_d_array(nums, target);
    println!("{}", flag);
}
