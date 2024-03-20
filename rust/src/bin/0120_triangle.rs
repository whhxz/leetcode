// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

use std::cmp::min;

struct Solution {}
impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        let high = triangle.len();
        for i in 1..high {
            triangle[i][0] += triangle[i - 1][0];
            let len = triangle[i].len();
            triangle[i][len - 1] += triangle[i - 1][len - 2];
            for j in 1..len - 1 {
                triangle[i][j] += min(triangle[i - 1][j - 1], triangle[i - 1][j])
            }
        }
        return *triangle[high - 1].iter().min().unwrap();
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let grid = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        let res = Solution::minimum_total(grid);
        assert_eq!(res, 11);
        let grid = vec![vec![-10]];
        let res = Solution::minimum_total(grid);
        assert_eq!(res, -10);
    }
}
fn main() {}
