struct Solution;

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        for i in a.iter().map(|item| item * item) {
            res.push(i);
        }
        res.sort();
        res
    }
}

pub fn test() {}
