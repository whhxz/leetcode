// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
use std::{cmp, collections::HashMap};
impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut left = 0;
        let mut right = 0;
        let mut res = 0;
        while right < nums.len() {
            let num = map.entry(nums[right]).or_insert(0);
            *num += 1;
            while *map.get(&nums[right]).unwrap() > k {
                let t = map.get_mut(&nums[left]).unwrap();
                *t -= 1;
                left += 1;
            }
            right += 1;
            res = cmp::max(res, right - left);
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1);
        assert_eq!(res, 2);
        let res = Solution::max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2);
        assert_eq!(res, 6);
    }
}
fn main() {}
