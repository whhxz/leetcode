#![allow(unused_variables, dead_code)]

use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        //前缀和
        let mut map = HashMap::new();
        let mut count = 0;
        let mut res = 0;
        for num in nums {
            let val = map.entry(count).or_insert(0);
            *val += 1;
            count += num;
            res += *map.get(&(count - goal)).unwrap_or(&0);
        }
        return res;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn demo1() {
        let arrays = vec![0, 0, 0, 0, 0];
        let res = Solution::num_subarrays_with_sum(arrays, 0);
        assert_eq!(res, 15);
        let arrays = vec![1, 0, 1, 0, 1];
        let res = Solution::num_subarrays_with_sum(arrays, 2);
        assert_eq!(res, 4);
    }
}
fn main() {}
