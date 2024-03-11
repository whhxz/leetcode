#![allow(unused_variables, dead_code)]

use std::collections::HashMap;
struct Solution {}
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut map = HashMap::new();
        for (i, v) in order.chars().enumerate() {
            map.insert(v, i);
        }
        let mut s: Vec<_> = s.chars().collect();
        let a = map.get(&'c').unwrap_or(&0);
        s.sort_by(|a, b| map.get(a).unwrap_or(&0).cmp(map.get(b).unwrap_or(&0)));
        return s.into_iter().collect();
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn demo1() {
        let res = Solution::custom_sort_string(String::from("cba"), String::from("abcd"));
        println!("{res}")
    }
}
fn main() {}
