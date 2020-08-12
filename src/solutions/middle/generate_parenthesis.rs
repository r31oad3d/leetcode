struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        if n == 0 {
            res.push("".to_string());
        } else {
            let mut c = 0;
            while c < n {
                for left in Solution::generate_parenthesis(c) {
                    for right in Solution::generate_parenthesis(n - 1 - c) {
                        res.push(
                            "(".to_owned()
                                + left.as_str()
                                + ")"
                                + right.as_str(),
                        );
                    }
                }
                c += 1;
            }
        }
        res
    }
}
