// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

use std::{
    collections::{BinaryHeap, HashMap},
    usize,
};


#[derive(Debug, PartialEq, Eq)]
struct Stata(char, usize);

impl Ord for Stata {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for Stata {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other).reverse())
    }
}

struct Solution {}
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut map = HashMap::new();
        for task in tasks {
            let mut val = map.entry(task).or_insert(Stata(task, 0));
            val.1 += 1;
        }
        for (_, v) in map {
            heap.push(v);
        }
        let mut before = '0';
        let mut res = 0;
        let mut s = n;
        while !heap.is_empty() {
            let data = heap.peek().unwrap();
            if data.0 == before{
                res += n;
                s = n;
            } else {
                if (s == 0){

                }
            }
        }
        return 0;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_add() {
        let res = Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2);
        assert_eq!(res, 2);
    }
}
fn main() {}
