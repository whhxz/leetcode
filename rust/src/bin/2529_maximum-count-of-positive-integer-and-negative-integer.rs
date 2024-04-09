// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        //TODO 二分法更快
        let (mut x, mut y) = (0, 0);
        for n in nums.iter() {
            if *n < 0 {
                x += 1;
            } else if *n > 0 {
                y += 1;
            }
        }
        std::cmp::max(x, y)
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]);
        assert_eq!(res, 3);
    }
}
fn main() {}
