// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]
struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let ss = s.split_ascii_whitespace();
        ss.last().unwrap().len() as i32
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::length_of_last_word(String::from("luffy is still joyboy"));
        assert_eq!(res, 6);
        let res = Solution::length_of_last_word(String::from("   fly me   to   the moon  "));
        assert_eq!(res, 4);
        let res = Solution::length_of_last_word(String::from("Hello World"));
        assert_eq!(res, 5);
    }
}
fn main() {}
