#![allow(unused_variables, dead_code)]

use std::collections::HashSet;
struct Solution {}
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();
        for n in nums1 {
            set.insert(n);
        }

        let mut res = HashSet::new();
        for n in nums2 {
            if set.contains(&n) {
                res.insert(n);
            }
        }
        return res.into_iter().collect();
    }
}
fn main() {}
