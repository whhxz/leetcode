// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
use std::cmp;
impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut matrix = matrix;
        let n = matrix.len();
        for i in 1..n {
            for j in 0..n {
                if j == 0 {
                    matrix[i][j] += cmp::min(matrix[i - 1][j], matrix[i - 1][j + 1])
                } else if j == n - 1 {
                    matrix[i][j] += cmp::min(matrix[i - 1][j - 1], matrix[i - 1][j])
                } else {
                    matrix[i][j] += cmp::min(
                        matrix[i - 1][j - 1],
                        cmp::min(matrix[i - 1][j], matrix[i - 1][j + 1]),
                    )
                }
            }
        }
        *matrix[n - 1].iter().min().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let matrix = [[-19]].to_vec().iter().map(|x| x.to_vec()).collect();
        let res = Solution::min_falling_path_sum(matrix);
        assert_eq!(res, -19);
        let matrix = [[-19, 57], [-40, -5]]
            .to_vec()
            .iter()
            .map(|x| x.to_vec())
            .collect();
        let res = Solution::min_falling_path_sum(matrix);
        assert_eq!(res, -59);
        let matrix = [[2, 1, 3], [6, 5, 4], [7, 8, 9]]
            .to_vec()
            .iter()
            .map(|x| x.to_vec())
            .collect();
        let res = Solution::min_falling_path_sum(matrix);
        assert_eq!(res, 13);
    }
}
fn main() {}
