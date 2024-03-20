// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
impl Solution {
    //f[m][n] = f[m-1][n] + f[m][n-1]
    pub fn unique_paths(m: i32,  n: i32) -> i32 {
        let mut m = m as usize;
        let mut n = n as usize;
        if m > n {
            m = m + n;
            n = m - n;
            m = m - n;
        }
        let mut nums1 = vec![1 as usize; m as usize];

        let mut nums2 = vec![1 as usize; m as usize];
        for i in 1..n {
            for j in 1..m {
                nums2[j] = nums2[j - 1] + nums1[j];
            }
            nums1 = nums2;
            nums2 = vec![1 as usize; m as usize];
        }
        return nums1[m-1] as i32;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::unique_paths(3, 1);
        assert_eq!(res, 1);
        let res = Solution::unique_paths(1, 7);
        assert_eq!(res, 1);
        let res = Solution::unique_paths(3, 7);
        assert_eq!(res, 28);
    }
}
fn main() {}
