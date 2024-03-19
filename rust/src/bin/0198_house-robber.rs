// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

use std::cmp;

struct Solution {}
impl Solution {
    //f(n)[不偷]=max(f(n-1)[偷], fn(n-1)[不偷])
    //f(n)[偷]=f(n-1)[不偷]+n
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut rooms: [i32; 2] = [0, 0];
        for num in nums {
            let t = rooms[1];
            rooms[1] = rooms[0] + num;
            rooms[0] = cmp::max(rooms[0], t);
        }
        return cmp::max(rooms[0], rooms[1]);
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::rob(vec![2, 1, 1, 2]);
        assert_eq!(res, 4);
        let res = Solution::rob(vec![1, 2, 3, 1]);
        assert_eq!(res, 4);
        let res = Solution::rob(vec![2, 7, 9, 3, 1]);
        assert_eq!(res, 12);
    }
}
fn main() {}
