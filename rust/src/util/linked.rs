#![allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T: Copy> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}
impl<T: Copy> ListNode<T> {
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

#[cfg(test)]
mod tests {
    use super::ListNode;

    #[test]
    fn from_vec() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4]);
        println!("{head:?}")
    }
}
