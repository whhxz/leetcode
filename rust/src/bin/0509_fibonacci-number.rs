// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
impl Solution {
    //斐波那契数 f(n) = f(n-1)+f(n-2)
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let (mut first, mut secend) = (0, 1);
        for i in 2..=n {
            secend += first;
            first = secend - first;
        }
        return secend;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_add() {
        let res = Solution::fib(4);
        assert_eq!(res, 3);
        let res = Solution::fib(3);
        assert_eq!(res, 2);
        let res = Solution::fib(2);
        assert_eq!(res, 1);
        let res = Solution::fib(1);
        assert_eq!(res, 1);
        let res = Solution::fib(0);
        assert_eq!(res, 0);
    }
}
fn main() {}
