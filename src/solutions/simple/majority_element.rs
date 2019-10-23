struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = nums[0];
        let mut count = 0_i32;
        for n in nums {
            if count == 0 {
                candidate = n;
            }
            if n == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate
    }


    pub fn majority_element_v1(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        if n == 1 {
            nums[0]
        } else {
            if n % 2 == 0 {
                nums[(n - 1) / 2]
            } else {
                nums[n / 2]
            }
        }
    }
}