// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

use std::cmp;

struct Solution {}
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable_by_key(|d| (d[0], d[1]));
        let mut res = 1;
        //可以只用一个参数，right，如果小于right就+1
        let (mut left, mut right) = (points[0][0], points[0][1]);
        for i in 0..points.len() {
            if points[i][0] <= right && points[i][1] >= left {
                left = cmp::max(points[i][0], left);
                right = cmp::min(points[i][1], right)
            } else {
                res += 1;
                left = points[i][0];
                right = points[i][1];
            }
        }
        return res;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_add() {
        let res =
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]);
        assert_eq!(res, 2);
        let res =
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]);
        assert_eq!(res, 4);
        let res =
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]);
        assert_eq!(res, 2);
    }
}
fn main() {}
