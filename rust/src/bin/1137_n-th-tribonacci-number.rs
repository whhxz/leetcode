// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
impl Solution {
    //第 N 个泰波那契数 f(n)=fn(n-3)+fn(n-2)+fn(n-1)
    pub fn tribonacci2(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let (mut i1, mut i2, mut i3) = (0, 1, 1);
        for i in 3..=n {
            i3 = i1 + i2 + i3;
            i2 = i3 - i2 - i1;
            i1 = i3 - i2 - i1;
        }
        return i3;
    }
    pub fn tribonacci(n: i32) -> i32 {
        let (mut i1, mut i2, mut i3) = (0, 1, 1);
        for i in 0..n {
            let t = i1 + i2 + i3;
            i1 = i2;
            i2 = i3;
            i3 = t;
        }
        return i1;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_add() {
        let res = Solution::tribonacci(25);
        assert_eq!(res, 1389537);
        let res = Solution::tribonacci(5);
        assert_eq!(res, 7);
        let res = Solution::tribonacci(4);
        assert_eq!(res, 4);
        let res = Solution::tribonacci(3);
        assert_eq!(res, 2);
        let res = Solution::tribonacci(2);
        assert_eq!(res, 1);
        let res = Solution::tribonacci(1);
        assert_eq!(res, 1);
        let res = Solution::tribonacci(0);
        assert_eq!(res, 0);
    }
    #[test]
    fn test_all() {
        for i in 0..=25 {
            let res = Solution::tribonacci(i);
            println!("{i} {res}")
        }
    }
}
fn main() {}
