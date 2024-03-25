// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for num in nums {
            let val = map.entry(num).or_insert(0);
            *val += 1;
        }
        let mut res = vec![];
        for (k, v) in map {
            if v == 2 {
                res.push(k);
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
        let res = Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]);
        assert_eq!(res, vec![2, 3]);
        let res = Solution::find_duplicates(vec![1, 1, 2]);
        assert_eq!(res, vec![1]);
        let res = Solution::find_duplicates(vec![]);
        assert_eq!(res, vec![]);
    }
}
fn main() {}
