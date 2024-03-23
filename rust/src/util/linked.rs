#![allow(dead_code)]

use std::fmt::Debug;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T>
where
    T: Copy + Debug,
{
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}
impl<T> ListNode<T>
where
    T: Copy + Debug,
{
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
    //TODO 只能反向，正向是否能实现
    pub fn from_vec(data: Vec<T>) -> Option<Box<ListNode<T>>> {
        if data.len() == 0 {
            return None;
        }
        let mut node = None;
        for i in (0..data.len()).rev() {
            let mut before = Box::new(ListNode::new(data[i]));
            before.next = node;
            node = Some(before);
        }
        return node;
    }
}

// impl<T> fmt::Display for ListNode<T>
// where
//     T: Copy + Debug,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self.val)
//     }
// }

#[cfg(test)]
mod tests {
    use super::ListNode;

    #[test]
    fn from_vec() {
        let node1 = ListNode::from_vec(vec![1, 2, 3, 4]);
        let node2 = ListNode::from_vec(vec![1, 2, 3, 4]);
        assert_eq!(node1, node2)
    }
}
