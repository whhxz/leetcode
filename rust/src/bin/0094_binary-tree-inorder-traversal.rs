// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]
type TreeNode = rust::util::tree::TreeNode<i32>;
struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        Self::deep(&root, &mut res);
        res
    }
    pub fn deep(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(v) = node {
            let n = &v.as_ref().borrow();
            Self::deep(&n.left, res);
            res.push(n.val);
            Self::deep(&n.right, res);
        }
    }
}
#[cfg(test)]
mod tests {
    type TreeNode = rust::util::tree::TreeNode<i32>;

    use crate::Solution;

    #[test]
    fn test1() {
        let root = TreeNode::from_vec(vec![1, i32::MIN, 2, 3]);
        let res = Solution::inorder_traversal(root);
        assert_eq!(res, vec![1, 3, 2]);
        let root = TreeNode::from_vec(vec![]);
        let res = Solution::inorder_traversal(root);
        assert_eq!(res, vec![]);
        let root = TreeNode::from_vec(vec![1]);
        let res = Solution::inorder_traversal(root);
        assert_eq!(res, vec![1]);
    }
}
fn main() {}
