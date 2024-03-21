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
    pub fn reverse_list1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut n1: Option<Box<ListNode>> = None;
        let mut n2 = head;
        while n2.is_some() {
            let mut current = n2.unwrap();
            let next = current.next;
            current.next = n1;
            n1 = Some(current);
            n2 = next;
        }
        return n1;
    }
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut n1: Option<Box<ListNode>> = None;
        let mut n2 = head;
        while n2.is_some() {
            let next = n2.as_mut().unwrap().next.take();
            n2.as_mut().unwrap().next = n1;
            n1 = n2;
            n2 = next;
        }
        return n1;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::reverse_list(None);
        assert_eq!(res, None);
    }
}
fn main() {}
