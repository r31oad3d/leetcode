struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let (mut sum, mut min, len) =
            (0_i32, i32::max_value(), nums.len() as i32);
        for i in nums.iter() {
            if *i < min {
                min = *i;
            }
            sum += *i;
        }
        sum - min * (len - 1) - min
    }
}
