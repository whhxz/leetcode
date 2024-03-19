// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

use std::cmp;

struct Solution {}
impl Solution {
    //f(n)=min(fn(n-1),fn(n-2))
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut c1 = 0;
        let mut c2 = 0;
        for i in 2..=cost.len() {
            let t = cmp::min(cost[i - 2] + c1, cost[i - 1] + c2);
            c1 = c2;
            c2 = t;
        }
        return c2;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]);
        assert_eq!(res, 6);
        let res = Solution::min_cost_climbing_stairs(vec![10, 15, 20]);
        assert_eq!(res, 15);
    }
}
fn main() {}
