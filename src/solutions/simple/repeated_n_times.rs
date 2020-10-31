struct Solution;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let n = a.len() / 2;

        for num in a {
            let cnt = map.entry(num).or_insert(0);
            *cnt += 1;
            if cnt == &n {
                return num;
            }
        }
        -1
    }
}

#[test]
fn test_it() {}
