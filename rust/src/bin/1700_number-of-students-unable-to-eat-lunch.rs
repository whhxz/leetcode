// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]
struct Solution {}
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let res = 0;
        let mut students = students;
        let mut sandwiches = sandwiches;
        loop {
            let (mut i, mut j) = (0, 0);
            let mut tmp_students = vec![];
            while i < students.len() && j < sandwiches.len() {
                if students[i] != sandwiches[j] {
                    tmp_students.push(students[i]);
                    i += 1;
                } else {
                    i += 1;
                    j += 1;
                }
            }
            if tmp_students.len() == students.len() {
                break;
            } else {
                students = tmp_students;
            }
            sandwiches = sandwiches[j..].to_vec();
        }
        students.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]);
        assert_eq!(res, 3);
        let res = Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]);
        assert_eq!(res, 0);
    }
}
fn main() {}
