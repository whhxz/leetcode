// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]
type ListNode = rust::util::linked::ListNode<i32>;
type TreeNode = rust::util::tree::TreeNode<i32>;
struct Solution {}
impl Solution {
    pub fn temp() -> i32 {
        return 1;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::temp();
        assert_eq!(res, 1);
    }
}
fn main() {}
