// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

use std::cmp;

struct Solution {}
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let len = intervals.len();
        let mut new_interval = new_interval;
        for i in 0..len {
            if intervals[i][1] < new_interval[0] {
                res.push(intervals[i].clone());
            } else if intervals[i][0] > new_interval[1] {
                res.push(new_interval.clone());
                new_interval = intervals[i].clone();
            } else if intervals[i][1] >= new_interval[0] || intervals[i][0] <= new_interval[1] {
                new_interval[0] = cmp::min(intervals[i][0], new_interval[0]);
                new_interval[1] = cmp::max(intervals[i][1], new_interval[1]);
            }
        }
        res.push(new_interval.clone());
        return res;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_add() {
        let res = Solution::insert(vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]], vec![4, 8]);
        // assert_eq!(res, 1);
    }
}
fn main() {}
