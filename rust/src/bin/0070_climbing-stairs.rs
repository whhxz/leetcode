#![allow(dead_code, unused_variables)]
struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let (mut i, mut j, mut k) = (1, 2, 3);
        while k <= n {
            j = i + j;
            i = j - i;
            k += 1;
        }
        //fn(n) = fn(n-1) + fn(n-2)
        return j;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_add() {
        let res = Solution::climb_stairs(45);
        assert_eq!(res, 1836311903);
        let res = Solution::climb_stairs(2);
        assert_eq!(res, 2);
        let res = Solution::climb_stairs(3);
        assert_eq!(res, 3);
    }
}

fn main() {}
