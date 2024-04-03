// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

use std::usize;

struct Solution {}
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let x = board.len();
        let y = board[0].len();
        let wc: Vec<char> = word.chars().collect();
        let mut walk = vec![vec![false; y]; x];
        for i in 0..x {
            for j in 0..y {
                if Solution::search(&board, &mut walk, &wc, 0, i as i32, j as i32) {
                    return true;
                }
            }
        }
        false
    }
    fn search(
        board: &Vec<Vec<char>>,
        walk: &mut Vec<Vec<bool>>,
        wc: &Vec<char>,
        wi: usize,
        i: i32,
        j: i32,
    ) -> bool {
        if wi == wc.len() {
            return true;
        }
        if i < 0 || j < 0 {
            return false;
        }
        let i = i as usize;
        let j = j as usize;
        if i >= board.len() || j >= board[0].len() {
            return false;
        }
        if walk[i][j] {
            return false;
        }
        if board[i][j] != wc[wi] {
            return false;
        }
        walk[i][j] = true;
        let i = i as i32;
        let j = j as i32;
        let res = Solution::search(board, walk, wc, wi + 1, i - 1, j)
            || Solution::search(board, walk, wc, wi + 1, i, j - 1)
            || Solution::search(board, walk, wc, wi + 1, i, j + 1)
            || Solution::search(board, walk, wc, wi + 1, i + 1, j);
        walk[i as usize][j as usize] = false;
        return res;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let board = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ]
        .map(|x| x.to_vec())
        .to_vec();
        let res = Solution::exist(board, String::from("ABCB"));
        assert_eq!(res, false);
        let board = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ]
        .map(|x| x.to_vec())
        .to_vec();
        let res = Solution::exist(board, String::from("SEE"));
        assert_eq!(res, true);
        let board = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ]
        .map(|x| x.to_vec())
        .to_vec();
        let res = Solution::exist(board, String::from("ABCCED"));
        assert_eq!(res, true);
    }
}
fn main() {}
