// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]

struct Solution {}
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut routes = vec![Vec::<usize>::new(); n as usize];
        for nums in &edges {
            if let Some(tos) = routes.get_mut(nums[0] as usize) {
                tos.push(nums[1] as usize);
            }
            if let Some(tos) = routes.get_mut(nums[1] as usize) {
                tos.push(nums[0] as usize);
            }
        }
        let mut works = vec![false; n as usize];
        Solution::dfs(&routes, &mut works, source as usize, destination as usize)
    }
    pub fn dfs(routes: &Vec<Vec<usize>>, works: &mut Vec<bool>, from: usize, to: usize) -> bool {
        if from == to {
            return true;
        }
        if works[from] == true {
            return false;
        }
        works[from] = true;
        let tos = &routes[from];
        for t in tos {
            if Solution::dfs(routes, works, *t, to) {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let edges = [[0, 1], [1, 2], [2, 0]].map(|x| x.to_vec()).to_vec();
        let res = Solution::valid_path(3, edges, 0, 2);
        assert_eq!(res, true);
    }
    #[test]
    fn test2() {
        let edges = [[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]]
            .map(|x| x.to_vec())
            .to_vec();
        let res = Solution::valid_path(3, edges, 0, 5);
        assert_eq!(res, false);
    }
}
fn main() {}
