#![allow(dead_code)]

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            if let Some(v) = map.get(&(target - nums[i])) {
                return vec![*v as i32, i as i32];
            }
            map.insert(nums[i], i);
        }
        return vec![];
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(res, vec![0, 1]);
        let res = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(res, vec![1, 2]);
        let res = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(res, vec![0, 1]);
    }
}
fn main() {}
