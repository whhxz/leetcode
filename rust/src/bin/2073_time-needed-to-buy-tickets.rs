// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]
struct Solution {}
impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let num = tickets[k as usize];
        let k = k as usize;
        for i in 0..tickets.len() {
            if i <= k {
                res += std::cmp::min(num, tickets[i]);
            } else {
                res += std::cmp::min(num - 1, tickets[i]);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::time_required_to_buy(vec![5, 1, 1, 1], 0);
        assert_eq!(res, 8);
        let res = Solution::time_required_to_buy(vec![2, 3, 2], 2);
        assert_eq!(res, 6);
    }
}
fn main() {}
