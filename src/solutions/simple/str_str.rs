struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.as_str().find(needle.as_str()) {
            Some(res) => res as i32,
            None => -1,
        }
    }
}
