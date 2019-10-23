struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut m = x ^ y;
        let mut count = 0;
        while m > 0 {
            if m&1 > 0 {
                count += 1;
            }
            m >>= 1;
        }
        count
    }
}