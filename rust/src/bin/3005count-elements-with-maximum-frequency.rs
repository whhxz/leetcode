#![allow(unused_variables, dead_code)]

use std::collections::HashMap;
struct Solution {}
impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for n in &nums {
            let v = map.entry(n).or_insert(0);
            *v += 1;
        }
        let mut max = 0;
        let mut res = 0;
        for (_, v) in &map {
            let v = *v;
            if v > max {
                max = v;
                res = 0;
            }
            if max == v {
                res += v
            }
        }
        return res;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![10, 12, 11, 9, 6, 19, 11];
        let res = Solution::max_frequency_elements(nums);
        assert_eq!(res, 2)
    }
    fn test_2() {
        let nums = vec![1,2,2,3,1,4];
        let res = Solution::max_frequency_elements(nums);
        assert_eq!(res, 4)
    }
}
fn main() {}
