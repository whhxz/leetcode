// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        if changed.len() % 2 == 1 {
            return vec![];
        }
        let mut changed = changed;
        changed.sort();
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in &changed {
            let val = map.entry(*n).or_insert(0);
            *val += 1;
        }
        let mut res = Vec::with_capacity(changed.len() / 2);
        if let Some(num) = map.get(&0) {
            if *num % 2 == 0 {
                res.extend(vec![0; (*num / 2) as usize]);
            } else {
                return vec![];
            }
        }

        for n in &changed {
            if *n == 0 {
                continue;
            }
            let num = map.get_mut(n).unwrap();
            if *num == 0 {
                continue;
            } else if *num < 0 {
                return vec![];
            }
            *num -= 1;
            res.push(*n);
            let double = map.get_mut(&(*n * 2));
            if let Some(double_num) = double {
                *double_num -= 1;
            } else {
                return vec![];
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
        let res = Solution::find_original_array(vec![1, 3, 4, 2, 6, 8]);
        assert_eq!(res, vec![1, 3, 4]);
    }

    #[test]
    fn test2() {
        let res = Solution::find_original_array(vec![6, 3, 0, 1]);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn test3() {
        let res = Solution::find_original_array(vec![1]);
        assert_eq!(res, vec![]);
    }
    #[test]
    fn test4() {
        let res = Solution::find_original_array(vec![0, 0, 0, 0]);
        assert_eq!(res, vec![0, 0]);
    }
}
fn main() {}
