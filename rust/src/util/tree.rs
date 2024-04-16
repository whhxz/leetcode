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
    pub fn from_vec2(data: Vec<i32>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        let len = data.len();
        if len == 0 {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(data[0])));
        let mut depth_nodes = vec![Some(root.clone())];
        let mut i = 1;
        while depth_nodes.len() > 0 && i < len {
            let mut ts = vec![];
            for node in &depth_nodes {
                if i >= len {
                    break;
                }
                let left = data
                    .get(i)
                    .filter(|x| **x != i32::MIN)
                    .map(|x| Rc::new(RefCell::new(TreeNode::new(*x))));
                let right = data
                    .get(i + 1)
                    .filter(|x| **x != i32::MIN)
                    .map(|x| Rc::new(RefCell::new(TreeNode::new(*x))));
                if let Some(node) = node {
                    let mut node = node.borrow_mut();
                    node.left = left.clone();
                    node.right = right.clone();
                }
                ts.push(left.clone());
                ts.push(right.clone());
                i += 2;
            }
            depth_nodes = ts;
        }
        Some(root)
    }

    pub fn from_vec(data: Vec<i32>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        let len = data.len();
        if len == 0 {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(data[0])));
        let mut depth_nodes = vec![root.clone()];
        let mut i = 1;
        while depth_nodes.len() > 0 && i < len {
            let mut ts = vec![];
            for node in &depth_nodes {
                if i >= len {
                    break;
                }
                let left = data
                    .get(i)
                    .filter(|x| **x != i32::MIN)
                    .map(|x| Rc::new(RefCell::new(TreeNode::new(*x))));
                let right = data
                    .get(i + 1)
                    .filter(|x| **x != i32::MIN)
                    .map(|x| Rc::new(RefCell::new(TreeNode::new(*x))));
                let mut node = node.borrow_mut();
                node.left = left.clone();
                node.right = right.clone();

                if let Some(node) = left {
                    ts.push(node);
                }
                if let Some(node) = right {
                    ts.push(node);
                }
                i += 2;
            }
            depth_nodes = ts;
        }
        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::TreeNode;

    // #[test]
    // fn from_vec() {
    //     let root: Option<Rc<RefCell<TreeNode<i32>>>> =
    //         TreeNode::<i32>::from_vec(vec![1, i32::MIN, 2, 3]);
    //     dbg!(&root);
    //     let root2: Option<Rc<RefCell<TreeNode<i32>>>> =
    //         TreeNode::<i32>::from_vec(vec![1, i32::MIN, 2, 3]);
    //     assert_eq!(root, root2);
    // }
    #[test]
    fn from_vec2() {
        let root: Option<Rc<RefCell<TreeNode<i32>>>> =
            TreeNode::<i32>::from_vec2(vec![1, i32::MIN, 2, 3]);
        let root2: Option<Rc<RefCell<TreeNode<i32>>>> =
            TreeNode::<i32>::from_vec2(vec![1, i32::MIN, 2, 3]);
        assert_eq!(root, root2);
    }

    #[test]
    fn from_vec3() {
        let null = i32::MIN;

        let t1 = TreeNode::<i32>::from_vec(vec![1, 4, null, 2, null, 3, 1]);
        let t2 = TreeNode::<i32>::from_vec2(vec![1, 4, null, 2, null, null, null, 3, 1]);
        assert_eq!(t1, t2);

        let t1 = TreeNode::<i32>::from_vec2(vec![4, 2, null, 3, 1]);
        let t2 = TreeNode::<i32>::from_vec(vec![4, 2, null, 3, 1]);
        assert_eq!(t1, t2);

        let t1 = TreeNode::<i32>::from_vec2(vec![4, 2, null, 1, 1, null, null, 3, null, null, 1]);
        let t2 = TreeNode::<i32>::from_vec(vec![4, 2, null, 1, 1, 3, null, null, 1]);
        assert_eq!(t1, t2);
    }
}
