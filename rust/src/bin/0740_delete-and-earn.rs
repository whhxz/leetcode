// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
use std::cmp;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap();
        let mut datas = vec![0; max as usize + 1];
        for n in nums {
            datas[n as usize] += n;
        }
        let mut before = [0, 0];
        for i in 1..=max {
            let t = cmp::max(before[0], before[1]);
            before[1] = cmp::max(before[1], before[0] + datas[i as usize]);
            before[0] = t;
        }
        return cmp::max(before[0], before[1]);
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::delete_and_earn(vec![2,2,3,3,3,4]);
        assert_eq!(res, 9);
        let res = Solution::delete_and_earn(vec![3, 4, 2]);
        assert_eq!(res, 6);
    }
}
fn main() {}
