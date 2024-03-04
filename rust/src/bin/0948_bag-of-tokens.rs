struct Solution {}

impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        if tokens.len() == 0 {
            return 0;
        }
        let mut j = tokens.len() - 1;
        let mut tokens = tokens;
        let mut power = power;
        let mut score = 0;
        tokens.sort();
        let mut i = 0;
        while i <= j {
            if tokens[i] <= power {
                score += 1;
                power -= tokens[i];
                i += 1;
            } else if tokens[j] > tokens[i] && score > 0 {
                power += tokens[j] - tokens[i];
                i += 1;
                j -= 1;
            } else {
                break;
            }
        }
        return score;
    }
}

fn main() {
    let tokens = vec![];
    let res = Solution::bag_of_tokens_score(tokens, 99);
    println!("{}", res)
}