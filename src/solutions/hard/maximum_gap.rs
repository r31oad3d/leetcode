struct Solution;

// should use another way to meet O(n)
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let mut prev = &-1_i32;
        let mut diff = 0_i32;
        for n in nums.iter().rev() {
            if prev != &-1 {
                let tmp = prev - n;
                if tmp > diff {
                    diff = tmp;
                }
            }
            prev = n;
        }
        diff
    }
}
