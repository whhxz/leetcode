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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut node = head.as_ref();
        let mut l = 0;
        while node.is_some() {
            node = node.unwrap().next.as_ref();
            l += 1;
        }
        let mut head = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut i = 0;
        let mut node = head.as_mut();
        while node.is_some() {
            let current = node.unwrap();
            if i + n == l {
                //TODO 赋值 as_mut take 学习
                let next = current.next.as_mut().unwrap().next.take();
                current.next = next;
            }
            node = current.next.as_mut();
            i += 1;
        }
        return head.unwrap().next;
    }
}
#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    #[test]
    fn test_add() {
        let n5 = Box::new(ListNode::new(5));
        let mut n4 = Box::new(ListNode::new(4));
        let mut n3 = Box::new(ListNode::new(3));
        let mut n2 = Box::new(ListNode::new(2));
        let mut n1 = Box::new(ListNode::new(1));
        n4.next = Some(n5);
        n3.next = Some(n4);
        n2.next = Some(n3);
        n1.next = Some(n2);
        println!("{:?}", n1);
        let res = Solution::remove_nth_from_end(Some(n1), 2);
        println!("{:?}", res)
    }
}
fn main() {}
