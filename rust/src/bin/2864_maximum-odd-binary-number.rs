#![allow(unused_variables, dead_code)]
struct Solution {}
impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut num = 0;
        for c in s.chars() {
            if c == '1' {
                num += 1;
            }
        }
        if num == 0 || s.len() == num {
            return s;
        }
        let mut res = "1".repeat(num - 1);
        res.push_str(&"0".repeat(s.len() - num));
        res.push_str(&"1");

        return res;
    }
}
fn main() {}
