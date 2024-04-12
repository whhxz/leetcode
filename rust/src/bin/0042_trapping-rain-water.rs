// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]
struct Solution {}
impl Solution {
    //记录每个节点左边最高和右边最高 l_max r_max
    // max(min(l_max, r_max) - num,0),小于0为0
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut rs = vec![0; height.len()];
        rs[height.len() - 1] = height[height.len() - 1];
        for i in (0..height.len() - 1).rev() {
            rs[i] = std::cmp::max(rs[i + 1], height[i])
        }
        let mut left = height[0];
        let mut res = 0;
        for i in 1..height.len() {
            left = std::cmp::max(left, height[i-1]);
            res += std::cmp::max(0,std::cmp::min(left, rs[i]) - height[i]);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::trap(vec![4,2,0,3,2,5]);
        assert_eq!(res, 9);
        let res = Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(res, 6);
    }
}
fn main() {}
