#![allow(unused_variables, dead_code)]
struct Solution {}
impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let mut left = ((1 + n) * n) / 2;
        let mut right = n;
        let mut i = n;
        while left >= right {
            if left == right {
                return i;
            }
            if left < right {
                return -1;
            }

            left -= i;
            i -= 1;
            right += i;
        }
        return -1;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn demo1() {
        let res = Solution::pivot_integer(1);
        assert_eq!(res, 1);
        let res = Solution::pivot_integer(8);
        assert_eq!(res, 6);
    }
}
fn main() {}
