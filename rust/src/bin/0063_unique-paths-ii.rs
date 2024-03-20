// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
impl Solution {
    //f[m][n]=f[m-1][n]+f[m][n-1]
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        if obstacle_grid[0][0] == 1 || obstacle_grid[m-1][n-1]==1{
            return 0;
        }

        for i in 1..m {
            if obstacle_grid[i - 1][0] == -1 || obstacle_grid[i][0] == 1 {
                obstacle_grid[i][0] = -1;
            } else {
                obstacle_grid[i][0] = 1;
            }
        }
        for i in 1..n {
            if obstacle_grid[0][i - 1] == -1 || obstacle_grid[0][i] == 1 {
                obstacle_grid[0][i] = -1;
            } else {
                obstacle_grid[0][i] = 1;
            }
        }
        obstacle_grid[0][0] = 1;
        for i in 1..m {
            for j in 1..n {
                if obstacle_grid[i][j] == 1 {
                    obstacle_grid[i][j] = -1;
                } else if obstacle_grid[i - 1][j] == -1 && obstacle_grid[i][j - 1] == -1 {
                    obstacle_grid[i][j] = -1;
                } else if obstacle_grid[i - 1][j] == -1 {
                    obstacle_grid[i][j] = obstacle_grid[i][j - 1];
                } else if obstacle_grid[i][j - 1] == -1 {
                    obstacle_grid[i][j] = obstacle_grid[i - 1][j];
                } else {
                    obstacle_grid[i][j] = obstacle_grid[i][j - 1] + obstacle_grid[i - 1][j];
                }
            }
        }
        if obstacle_grid[m - 1][n - 1] == -1 {
            0
        } else {
            obstacle_grid[m - 1][n - 1]
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let grid = [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
            .map(|x| x.to_vec())
            .to_vec();
        let res = Solution::unique_paths_with_obstacles(grid);
        assert_eq!(res, 2);
        let grid = [[0, 1], [0, 0]].map(|x| x.to_vec()).to_vec();
        let res = Solution::unique_paths_with_obstacles(grid);
        assert_eq!(res, 1);
        let grid = [[0], [0], [0]].map(|x| x.to_vec()).to_vec();
        let res = Solution::unique_paths_with_obstacles(grid);
        assert_eq!(res, 1);
        let grid = [[0], [1], [0]].map(|x| x.to_vec()).to_vec();
        let res = Solution::unique_paths_with_obstacles(grid);
        assert_eq!(res, 0);
        let grid = [[0, 0, 0]].map(|x| x.to_vec()).to_vec();
        let res = Solution::unique_paths_with_obstacles(grid);
        assert_eq!(res, 1);
        let grid = [[0, 1, 0]].map(|x| x.to_vec()).to_vec();
        let res = Solution::unique_paths_with_obstacles(grid);
        assert_eq!(res, 0);
        let grid = [[1]].map(|x| x.to_vec()).to_vec();
        let res = Solution::unique_paths_with_obstacles(grid);
        assert_eq!(res, 0);
        let grid = [[0]].map(|x| x.to_vec()).to_vec();
        let res = Solution::unique_paths_with_obstacles(grid);
        assert_eq!(res, 1);
        let grid = [[1,0]].map(|x| x.to_vec()).to_vec();
        let res = Solution::unique_paths_with_obstacles(grid);
        assert_eq!(res,0);
    }
}
fn main() {}
