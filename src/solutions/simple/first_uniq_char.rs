struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        for (i, c) in s.chars().enumerate() {
            let ref mut value = map.entry(c).or_insert((vec![], 0));
            value.0.push(i);
            value.1 += 1;
        }
        let mut res = usize::max_value();
        for (k, v) in map.iter() {
            if v.1 == 1 {
                if res > v.0[0] {
                    res = v.0[0];
                }
            }
        }
        res as i32
    }
}
