// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]
type TreeNode = rust::util::tree::TreeNode<i32>;
struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
use std::vec;

impl Solution {
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let mut node = TreeNode::new(val);
            node.left = root;
            return Some(Rc::new(RefCell::new(node)));
        }
        let mut depth_nodes = vec![root.clone().unwrap()];
        let mut depth = depth;
        while depth != 2 {
            let mut ts = vec![];
            for node in &depth_nodes {
                let node = node.borrow();
                if let Some(left) = node.left.clone() {
                    ts.push(left);
                }
                if let Some(right) = node.right.clone() {
                    ts.push(right);
                }
            }
            depth -= 1;
            depth_nodes = ts;
        }
        for node in &depth_nodes {
            let mut node = node.borrow_mut();
            let left = node.left.clone();
            let right = node.right.clone();

            let mut new_left = TreeNode::new(val);
            new_left.left = left;

            let mut new_right = TreeNode::new(val);
            new_right.right = right;

            node.left = Some(Rc::new(RefCell::new(new_left)));
            node.right = Some(Rc::new(RefCell::new(new_right)));
        }
        root
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;
    type TreeNode = rust::util::tree::TreeNode<i32>;

    #[test]
    fn test1() {
        let null: i32 = i32::MIN;

        let root = TreeNode::from_vec(vec![4, 2, null, 3, 1]);
        let res = Solution::add_one_row(root, 1, 3);
        assert_eq!(
            res,
            TreeNode::from_vec(vec![4, 2, null, 1, 1, 3, null, null, 1])
        );
        let root = TreeNode::from_vec(vec![4, 2, null, 3, 1]);
        let res = Solution::add_one_row(root, 1, 1);
        assert_eq!(
            res,
            dbg!(TreeNode::from_vec(vec![1, 4, i32::MIN, 2, null, 3, 1]))
        );
    }
}
fn main() {}
