// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
impl Solution {
    pub fn final_string(s: String) -> String {
        let mut res = String::new();
        for (i,c) in s.chars().enumerate() {
            if c == 'i'{
                res = res.chars().rev().collect();    
            } else {
                res.push(c);
            }
        }
        return res;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::final_string(String::from("poiinteri"));
        assert_eq!(res, "retnop");
        let res = Solution::final_string(String::from("poiinter"));
        assert_eq!(res, "ponter");
        let res = Solution::final_string(String::from("string"));
        assert_eq!(res, "rtsng");
    }
    #[test]
    fn test2() {
        let a: String = "1".chars().rev().collect();
    }
}
fn main() {}
