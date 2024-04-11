// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]
struct Solution {}
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut k = k;
        let mut stack = vec![];
        for c in num.chars() {
            while k > 0 && stack.len() > 0 {
                if c < stack[stack.len() - 1] {
                    stack.pop();
                    k -= 1;
                } else {
                    break;
                }
            }
            stack.push(c)
        }
        let res: String = stack.into_iter().collect();
        let res = res.trim_start_matches('0');
        if res.len() as i32 <= k {
            return String::from("0");
        } else {
            return (&res[..res.len() - k as usize]).to_string();
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::remove_kdigits(String::from("112"), 1);
        assert_eq!(res, "11");
        let res = Solution::remove_kdigits(String::from("0"), 2);
        assert_eq!(res, "0");
        let res = Solution::remove_kdigits(String::from("10"), 2);
        assert_eq!(res, "0");
        let res = Solution::remove_kdigits(String::from("10200"), 1);
        assert_eq!(res, "200");
        let res = Solution::remove_kdigits(String::from("1432219"), 3);
        assert_eq!(res, "1219");
    }
}
fn main() {}
