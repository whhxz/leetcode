#![allow(dead_code, unused_variables)]
struct Solution {}
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut j = s.len() - 1;
        let mut i = 0;
        while i < j && bytes[i] == bytes[j] {
            i += 1;
            while i <= j && bytes[i - 1] == bytes[i] {
                i += 1;
            }
            j -= 1;
            while i <= j && bytes[j] == bytes[j + 1] {
                j -= 1;
            }
        }
        if i > j {
            return 0;
        }
        return (j - i + 1) as i32;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let res = Solution::minimum_length(String::from("ca"));
        assert_eq!(res, 2);
        let res = Solution::minimum_length(String::from("cabaabac"));
        assert_eq!(res, 0);
        let res = Solution::minimum_length(String::from("aabccabba"));
        assert_eq!(res, 3);
        let res = Solution::minimum_length(String::from("a"));
        assert_eq!(res, 1);
        let res = Solution::minimum_length(String::from("abba"));
        assert_eq!(res, 0);
        let res = Solution::minimum_length(String::from("abbba"));
        assert_eq!(res, 0);
        let res = Solution::minimum_length(String::from("abbcba"));
        assert_eq!(res, 1);
        let res = Solution::minimum_length(String::from("bbbbbbbbbbbbbbbbbbb"));
        assert_eq!(res, 0);
    }
}
fn main() {
    let tmp = String::from("abc");
    println!("{:?}", tmp.as_bytes())
}
