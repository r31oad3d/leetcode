struct Solution;
impl Solution {
    pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
        #[inline]
        fn is_odd_number(num: &i32) -> bool {
            num | 0x01 == *num
        }
        if nums.is_empty() {
            return nums;
        }
        let mut nums = nums;
        let (mut i, mut j) = (0_usize, nums.len() - 1);
        while i < j {
            let (num1, num2) = (&nums[i], &nums[j]);
            match (is_odd_number(num1), is_odd_number(num2)) {
                (true, true) => {
                    i += 1;
                }
                (true, false) => {
                    i += 1;
                    j -= 1;
                }
                (false, false) => {
                    j -= 1;
                }
                (false, true) => {
                    nums.swap(i, j);
                    i += 1;
                    j -= 1;
                }
            };
        }
        nums
    }
}
