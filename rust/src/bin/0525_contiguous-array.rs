#![allow(unused_variables, dead_code)]

use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                count += 1;
            } else {
                count -= 1;
            }
            let val = map.get(&count);
            if val.is_some() {
                let t = (i as i32) - val.unwrap();
                if t > res {
                    res = t;
                }
            } else {
                map.insert(count, i as i32);
            }
        }
        return res;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn demo1() {
        let res = Solution::find_max_length(vec![0, 1]);
        assert_eq!(res, 2);
        let res = Solution::find_max_length(vec![0, 1, 0]);
        assert_eq!(res, 2);
    }
}

fn main() {}
