#![allow(dead_code)]
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
    use rust::util::linked::ListNode;

    use crate::Solution;

    #[test]
    fn test1() {
        let node = ListNode::from_vec(vec![1, 2, 2, 1]);
        let res = Solution::is_palindrome(node);
        assert!(res);
        let node = ListNode::from_vec(vec![1, 2]);
        let res = Solution::is_palindrome(node);
        assert!(!res);
    }
}
fn main() {}
