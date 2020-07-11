pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i_i, i) in nums.iter().enumerate() {
            for (i_j, j) in nums.iter().enumerate() {
                if i_i == i_j {
                    continue;
                }
                if i + j == target {
                    return vec![i_i as i32, i_j as i32];
                }
            }
        }
        Vec::new()
    }
}

pub fn test() {}
