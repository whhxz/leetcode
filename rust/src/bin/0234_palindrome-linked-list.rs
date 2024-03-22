#![allow(dead_code)]
// Definition for singly-linked list.

type ListNode = rust::util::linked::ListNode<i32>;
struct Solution {}
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut head = head;
        let mut node = head.as_mut();
        let mut datas = vec![];
        while node.is_some() {
            let t = node.unwrap();
            datas.push(t.val);
            node = t.next.as_mut();
        }
        let mut i = 0;
        node = head.as_mut();
        while node.is_some() && i <= datas.len() / 2 {
            let t = node.unwrap();
            if t.val != datas[datas.len() - i - 1] {
                return false;
            }
            i += 1;
            node = t.next.as_mut();
        }
        return true;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::is_palindrome(None);
        assert_eq!(res, true);
    }
}
fn main() {}
