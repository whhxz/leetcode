// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
use std::cmp;
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut nums = vec![0; matrix[0].len()];
        let mut res = 0;
        for i in 0..matrix.len() {
            let mut tmp = vec![0; matrix[0].len()];
            for j in 0..matrix[i].len() {
                if matrix[i][j] != '1' {
                    continue;
                }
                if j != 0 {
                    tmp[j] = cmp::min(tmp[j - 1], cmp::min(nums[j - 1], nums[j])) + 1;
                    res = cmp::max(res, tmp[j])
                } else {
                    tmp[j] = 1;
                    res = cmp::max(res, 1);
                }
            }
            nums = tmp;
        }
        res * res
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let matrix = [
            ['1','1'],
            ['1','1'],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        let res = Solution::maximal_square(matrix);
        assert_eq!(res, 4);
        let matrix = [
            ['0'],
            ['0'],
            ['0'],
            ['0'],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        let res = Solution::maximal_square(matrix);
        assert_eq!(res, 0);
        let matrix = [['0','1'],['1','0']]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        let res = Solution::maximal_square(matrix);
        assert_eq!(res, 1);
        let matrix = [
            ['1', '0', '1', '0', '0'],
            ['1', '0', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '0', '0', '1', '0'],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        let res = Solution::maximal_square(matrix);
        assert_eq!(res, 4);
    }
}
fn main() {}
