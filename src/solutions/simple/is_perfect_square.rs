struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut num = num;
        let mut sum = 1_i32;
        while num > 0 {
            num -= sum;
            sum += 2;
        }
        num == 0
    }
}