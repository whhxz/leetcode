// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]
struct Solution {}
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let il = grid.len() - 1;
        let jl = grid[0].len() - 1;
        for i in 0..=il {
            for j in 0..=jl {
                if grid[i][j] == 1 {
                    //左
                    if j == 0 || grid[i][j - 1] == 0 {
                        res += 1;
                    }
                    //上
                    if i == 0 || grid[i - 1][j] == 0 {
                        res += 1;
                    }
                    //右
                    if j == jl || grid[i][j + 1] == 0 {
                        res += 1;
                    }
                    //下
                    if i == il || grid[i + 1][j] == 0 {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let grid = [[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [1, 1, 0, 0]]
            .map(|x| x.to_vec())
            .to_vec();
        let res = Solution::island_perimeter(grid);
        assert_eq!(res, 16);
    }
    #[test]
    fn test2() {
        let grid = [[1]].map(|x| x.to_vec()).to_vec();
        let res = Solution::island_perimeter(grid);
        assert_eq!(res, 4);
    }
    #[test]
    fn test3() {
        let grid = [[1, 0]].map(|x| x.to_vec()).to_vec();
        let res = Solution::island_perimeter(grid);
        assert_eq!(res, 4);
    }
}
fn main() {}
