// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]
struct Solution {}
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;
        let mut grid = grid;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    Solution::flood(&mut grid, i, j);
                    res += 1;
                }
            }
        }
        res
    }

    pub fn flood(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let a = grid.get_mut(i).and_then(|x| x.get_mut(j));
        if a.is_none() {
            return;
        }
        let a = a.unwrap();
        if *a == '0' {
            return;
        }
        *a = '0';
        Solution::flood(grid, i + 1, j);
        Solution::flood(grid, i, j + 1);
        if j != 0 {
            Solution::flood(grid, i, j - 1);
        }
        if i != 0{
            Solution::flood(grid, i-1, j);
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let grid = [
            ['1', '1', '1', '1', '0'],
            ['1', '1', '0', '1', '0'],
            ['1', '1', '0', '0', '0'],
            ['0', '0', '0', '0', '0'],
        ]
        .map(|x| x.to_vec())
        .to_vec();
        let res = Solution::num_islands(grid);
        assert_eq!(res, 1);
    }
    #[test]
    fn test2() {
        let grid = [
            ['1', '1', '0', '0', '0'],
            ['1', '1', '0', '0', '0'],
            ['0', '0', '1', '0', '0'],
            ['0', '0', '0', '1', '1'],
        ]
        .map(|x| x.to_vec())
        .to_vec();
        let res = Solution::num_islands(grid);
        assert_eq!(res, 3);
    }
    #[test]
    fn test3() {
        let grid = [
            ['1', '0', '1', '1', '1'],
            ['1', '0', '1', '0', '1'],
            ['1', '1', '1', '0', '1'],
        ]
        .map(|x| x.to_vec())
        .to_vec();
        let res = Solution::num_islands(grid);
        assert_eq!(res, 1);
    }
}
fn main() {}
