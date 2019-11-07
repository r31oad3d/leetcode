struct Solution;


impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut res = 0_i32;
        let mut tmp = 0_i32;
        for n in nums {
            if n == 1 {
                tmp += n;
            } else {
                if tmp > res {
                    res = tmp;
                }
                tmp = 0;
            }
        }
        if tmp > res {
            res = tmp;
        }
        res
    }
}