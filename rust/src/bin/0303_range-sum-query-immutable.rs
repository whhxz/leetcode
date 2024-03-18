// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct NumArray {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut nums = nums;
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        return NumArray { data: nums };
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            self.data[right as usize]
        } else {
            self.data[right as usize] - self.data[left as usize - 1]
        }
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
#[cfg(test)]
mod tests {
    use crate::NumArray;

    #[test]
    fn test_add() {
        let num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        let res = num_array.sum_range(0, 2);
        assert_eq!(res, 1);
        let res = num_array.sum_range(2, 5);
        assert_eq!(res, -1);
        let res = num_array.sum_range(0, 5);
        assert_eq!(res,-3);
    }
}
fn main() {}
