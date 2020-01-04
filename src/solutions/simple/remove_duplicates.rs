struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0{
            return 0_i32;
        }
        if nums.len() == 1{
            return 1_i32;
        }
        let (mut i, mut j) = (0_usize, 1_usize);
        loop {
            let i_value = nums[i];
            let j_value = nums[j];
            if i_value == j_value {
                j += 1;
            } else if j_value > i_value {
                if j == i + 1{
                    i += 1;
                    j += 1;
                } else {
                    i += 1;
                    nums.swap(i,j);
                    j += 1;
                }
            }
            if j == nums.len() {
                break;
            }
        }
        (i+1) as i32
    }
}