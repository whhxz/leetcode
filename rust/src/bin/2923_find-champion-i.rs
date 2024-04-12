// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
use std::collections::HashSet;
impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let mut set = HashSet::new();
        for (i, nums) in grid.iter().enumerate() {
            for (j, num) in nums.iter().enumerate() {
                if *num == 1 {
                    set.insert(j);
                }
            }
        }
        for i in 0..grid.len() {
            if !set.contains(&i) {
                return i as i32;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let grid = [[0, 0, 1], [1, 0, 1], [0, 0, 0]]
            .map(|x| x.to_vec())
            .to_vec();
        let res = Solution::find_champion(grid);
        assert_eq!(res, 1);
        let grid = [[0, 1], [0, 0]].map(|x| x.to_vec()).to_vec();
        let res = Solution::find_champion(grid);
        assert_eq!(res, 0);
    }
}
fn main() {}
