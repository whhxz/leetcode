// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

use std::{cell::RefCell, rc::Rc};

type ListNode = rust::util::linked::ListNode<i32>;
type TreeNode = rust::util::tree::TreeNode<i32>;
struct Solution {}
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(parent: i32, node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if node.is_none() {
                return 0;
            }
            let node = node.unwrap();
            let node = node.borrow();
            let parent = 10 * parent + node.val;
            let left = node.left.clone();
            let right = node.right.clone();
            if left.is_none() && right.is_none(){
                return parent;
            }
            dfs(parent, node.left.clone()) + dfs(parent, node.right.clone())
        }
        dfs(0, root)
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;
    type TreeNode = rust::util::tree::TreeNode<i32>;

    #[test]
    fn test1() {
        let tree = TreeNode::from_vec(vec![4, 9, 0, i32::MIN, 1]);
        let res = Solution::sum_numbers(tree);
        assert_eq!(res, 531);
        let tree = TreeNode::from_vec(vec![1, 2, 3]);
        let res = Solution::sum_numbers(tree);
        assert_eq!(res, 25);
        let tree = TreeNode::from_vec(vec![4, 9, 0, 5, 1]);
        let res = Solution::sum_numbers(tree);
        assert_eq!(res, 1026);
    }
}
fn main() {}
