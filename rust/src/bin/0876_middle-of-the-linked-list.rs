// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution {}
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l = 0;
        let mut node = &head;
        while node.is_some() {
            l += 1;
            node = &node.as_ref().unwrap().next;
        }
        let l = l >> 1;
        let mut node = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        for i in 0..=l {
            node = node.unwrap().as_mut().next.take();
        }
        return node;
    }
}
#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    #[test]
    fn test_1() {
        let n5 = Box::new(ListNode::new(5));
        let mut n4 = Box::new(ListNode::new(4));
        let mut n3 = Box::new(ListNode::new(3));
        let mut n2 = Box::new(ListNode::new(2));
        let mut n1 = Box::new(ListNode::new(1));
        n4.next = Some(n5);
        n3.next = Some(n4);
        n2.next = Some(n3);
        n1.next = Some(n2);
        let res = Solution::middle_node(Some(n1));
        assert_eq!(res.unwrap().val, 3)
    }
    #[test]
    fn test_2() {
        let n6 = Box::new(ListNode::new(6));
        let mut n5 = Box::new(ListNode::new(5));
        let mut n4 = Box::new(ListNode::new(4));
        let mut n3 = Box::new(ListNode::new(3));
        let mut n2 = Box::new(ListNode::new(2));
        let mut n1 = Box::new(ListNode::new(1));
        n5.next = Some(n6);
        n4.next = Some(n5);
        n3.next = Some(n4);
        n2.next = Some(n3);
        n1.next = Some(n2);
        let res = Solution::middle_node(Some(n1));
        assert_eq!(res.unwrap().val, 4)
    }
    #[test]
    fn test_3() {
        let n1 = Box::new(ListNode::new(1));
        let res = Solution::middle_node(Some(n1));
        assert_eq!(res.unwrap().val, 1)
    }
}
fn main() {}
