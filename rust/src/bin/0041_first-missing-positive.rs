// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
use std::collections::HashSet;
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for n in &nums {
            set.insert(*n);
        }
        for i in 1..=nums.len() {
            if !set.contains(&(i as i32)) {
                return i as i32;
            }
        }
        return nums.len() as i32 + 1;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::first_missing_positive(vec![1]);
        assert_eq!(res, 2);
        let res = Solution::first_missing_positive(vec![1, 2, 0]);
        assert_eq!(res, 3);
        let res = Solution::first_missing_positive(vec![3, 4, -1, 1]);
        assert_eq!(res, 2);
        let res = Solution::first_missing_positive(vec![7, 8, 9, 11, 12]);
        assert_eq!(res, 1);
    }
}
fn main() {}
