struct Solution;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashSet;
        let mut set = HashSet::<i32>::new();
        let mut i = 0_usize;
        let k = k as usize;
        while let Some(n) = nums.get(i) {
            if set.contains(n) {
                return true;
            }
            set.insert(n.clone());
            if set.len() > k {
                set.remove(&nums[i - k]);
            }
            i += 1;
        }
        return false;
    }
}
