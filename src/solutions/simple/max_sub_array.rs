struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = i32::min_value();
        let mut tmp = 0_i32;
        for i in nums {
            if i < 0 && i > res {
                res = i;
            }
            tmp += i;
            if tmp < 0 {
                tmp = 0;
            } else {
                if res < tmp {
                    res = tmp;
                }
            }
        }
        res
    }
}
