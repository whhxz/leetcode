#![allow(unused_variables, dead_code)]
struct Solution {}
impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        let mid = target / 2;
        let before = (if n <= mid { n } else { mid }) as i64;
        let mut res = before * (before + 1) / 2;
        if n > mid {
            let after = (target + n - mid - 1) as i64;
            let target = target as i64;
            let num = (n - mid) as i64;
            res += (after + target) * num / 2
        }
        let base = i64::pow(10, 9) + 7;
        return (res % (base)) as i32;
    }
}
#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn demo1() {
        let res = Solution::minimum_possible_sum(2, 3);
        assert_eq!(res, 4);
        let res = Solution::minimum_possible_sum(3, 3);
        assert_eq!(res, 8);
        let res = Solution::minimum_possible_sum(1, 1);
        assert_eq!(res, 1);
        let res = Solution::minimum_possible_sum(16, 6);
        assert_eq!(res, 162);
    }
    #[test]
    fn demo2() {
        let res = Solution::minimum_possible_sum(16, 6);
        assert_eq!(res, 162);
    }
}
fn main() {}
