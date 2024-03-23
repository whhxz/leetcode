#![allow(dead_code)]

use std::{cell::RefCell, fmt::Debug, rc::Rc};
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T>
where
    T: Copy + Debug,
{
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Copy + Debug,
{
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    pub fn from_vec(data: Vec<i32>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        let len = data.len();
        if len == 0 {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(data[0])));
        let mut nodes = vec![Rc::clone(&root)];
        let mut i = 0;
        while nodes.len() > 0 && i < len {
            let mut ts = vec![];
            for node in &nodes {
                let mut node = node.borrow_mut();
                i += 1;
                if i == len {
                    break;
                }
                if data[i] != i32::MIN {
                    let left = Rc::new(RefCell::new(TreeNode::new(data[i])));
                    node.left = Some(Rc::clone(&left));
                    ts.push(left);
                }
                i += 1;
                if i == len {
                    break;
                }
                if data[i] != i32::MIN {
                    let right = Rc::new(RefCell::new(TreeNode::new(data[i])));
                    node.right = Some(Rc::clone(&right));
                    ts.push(right);
                }
            }
            nodes = ts;
        }
        return Some(root);
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::TreeNode;

    #[test]
    fn from_vec() {
        let root: Option<Rc<RefCell<TreeNode<i32>>>> =
            TreeNode::<i32>::from_vec(vec![1, i32::MIN, 2, 3]);
        dbg!(&root);
        let root2: Option<Rc<RefCell<TreeNode<i32>>>> =
            TreeNode::<i32>::from_vec(vec![1, i32::MIN, 2, 3]);
        assert_eq!(root, root2);
    }
}
