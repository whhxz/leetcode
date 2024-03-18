#![allow(unused_variables, dead_code)]

struct Solution {}
impl Solution {
    pub fn product_except_self1(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut left = Vec::with_capacity(len);
        left.push(nums[0]);
        let mut right_reverse = Vec::with_capacity(len);
        right_reverse.push(nums[len - 1]);
        for i in 1..=len - 1 {
            left.push(nums[i] * left[i - 1]);
            right_reverse.push(nums[len - 1 - i] * right_reverse[i - 1]);
        }
        let mut res = Vec::with_capacity(len);
        res.push(right_reverse[len - 2]);
        for i in 1..len - 1 {
            res.push(left[i - 1] * right_reverse[len - i - 2])
        }
        res.push(left[len - 2]);
        return res;
    }
    //单数值处理
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut left = vec![1; len];
        for i in 1..len {
            left[i] = nums[i-1] * left[i - 1]
        }
        let mut res = vec![1; len];
        let mut right = 1;
        for i in (0..len).rev() {
            res[i] = left[i] * right;
            right *= nums[i]
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn demo1() {
        let arrays = vec![1, 2, 3, 4];
        let res = Solution::product_except_self(arrays);
        assert_eq!(&[24, 12, 8, 6], &res[..]);
        let arrays = vec![-1, 1, 0, -3, 3];
        let res = Solution::product_except_self(arrays);
        assert_eq!(&[0, 0, 9, 0, 0], &res[..]);
    }
    #[test]
    fn demo2() {
        let mut d = Vec::with_capacity(2);
        d.push(2);
        println!("{}", d.len());
    }
}
fn main() {}
