#![allow(unused_variables, dead_code)]

struct Solution {}
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                return nums1[i];
            } else if nums1[i] > nums2[j] {
                j += 1;
            } else {
                i += 1;
            }
        }
        return -1;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn demo1() {
        let res = Solution::get_common(vec![1, 2, 3], vec![2, 4]);
        assert_eq!(res, 2)
    }
    #[test]
    fn demo2() {
        let res = Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]);
        assert_eq!(res, 2)
    }
    #[test]
    fn demo3() {
        let res = Solution::get_common(vec![2,4], vec![1,2]);
        assert_eq!(res, 2)
    }
}

fn main() {}
