// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut maps = HashMap::new();
        let mut mapt = HashMap::new();
        let mut t = t.chars();
        for (i, c) in s.char_indices() {
            let tc = t.next().unwrap();
            let val = maps.get(&c);
            if val.is_some() && *val.unwrap() != tc {
                return false;
            }
            let val = mapt.get(&tc);
            if val.is_some() && *val.unwrap() != c {
                return false;
            }
            maps.insert(c, tc);
            mapt.insert(tc, c);
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::is_isomorphic(String::from("badc"), String::from("baba"));
        assert_eq!(res, false);
        let res = Solution::is_isomorphic(String::from("egg"), String::from("add"));
        assert_eq!(res, true);
        let res = Solution::is_isomorphic(String::from("foo"), String::from("bar"));
        assert_eq!(res, false);
        let res = Solution::is_isomorphic(String::from("paper"), String::from("title"));
        assert_eq!(res, true);
    }
}
fn main() {}
