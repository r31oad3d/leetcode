struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        use std::cmp::max;
        let mut v: Vec<char> = s.chars().collect();
        let mut res = 0;
        let mut stack: Vec<i32> = vec![];
        stack.push(-1);
        for (i, c) in v.iter().enumerate() {
            match c {
                '(' => { stack.push(i as i32); },
                ')' => {
                    stack.pop();
                    if stack.is_empty() {
                        stack.push(i as i32);
                    } else {
                        res = max(res, i as i32 - stack.last().unwrap().clone());
                    }
                },
                _ => { panic!("wtf?!") },
            }
        }
        res
    }
}