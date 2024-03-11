#![allow(unused_variables, dead_code)]
struct Solution {}
impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let title: Vec<&str> = title.split(" ").collect();
        let len = &title.len();
        let mut res: Vec<String> = Vec::with_capacity(*&title.len());
        for (i, v) in title.into_iter().enumerate() {
            if v.len() == 1 || v.len() == 2 {
                let x = v.to_ascii_lowercase();
                res.push(x);
            } else {
                let mut first = v[0..1].to_uppercase();
                let second = v[1..].to_lowercase();
                first.push_str(&second);
                res.push(first);
            }
        }
        return res.join(" ");
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn demo1() {
        let res = Solution::capitalize_title(String::from("capiTalIze tHe titLe"));
        assert_eq!(res, String::from("Capitalize The Title"))
    }
    #[test]
    fn demo2() {
        let res = Solution::capitalize_title(String::from("A First leTTeR of EACH Word"));
        assert_eq!(res, String::from("a First Letter of Each Word"))
    }
}
fn main() {}
