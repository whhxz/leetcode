// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

use std::cmp::min;

struct Solution {}
impl Solution {
    //f[m][n]=min(f[m-1][n],f[m][n-1])
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        for i in 1..n {
            grid[0][i] += grid[0][i - 1]
        }
        for i in 1..m {
            grid[i][0] += grid[i - 1][0]
        }
        for i in 1..m {
            for j in 1..n {
                grid[i][j] += min(grid[i - 1][j], grid[i][j - 1])
            }
        }
        return grid[m - 1][n - 1];
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let grid = [[1], [4]].map(|x| x.to_vec()).to_vec();
        let res = Solution::min_path_sum(grid);
        assert_eq!(res, 5);
        let grid = [[1, 2, 3]].map(|x| x.to_vec()).to_vec();
        let res = Solution::min_path_sum(grid);
        assert_eq!(res, 6);
        let grid = [[1, 2, 3], [4, 5, 6]].map(|x| x.to_vec()).to_vec();
        let res = Solution::min_path_sum(grid);
        assert_eq!(res, 12);
        let grid = [[1, 3, 1], [1, 5, 1], [4, 2, 1]]
            .map(|x| x.to_vec())
            .to_vec();
        let res = Solution::min_path_sum(grid);
        assert_eq!(res, 7);
    }
}
fn main() {}
