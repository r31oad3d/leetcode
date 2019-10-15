struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        use std::collections::{HashMap,HashSet};
        let mut arr = arr;
        let mut hash_map = HashMap::new();
        for i in arr {
            *hash_map.entry(i).or_insert(0) += 1;
        }
        let value_unique :HashSet<i32> = hash_map.values().cloned().collect();
        hash_map.len() == value_unique.len()
    }
}