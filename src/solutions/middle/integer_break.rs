struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n <= 3 { return n - 1; }
        let tmp1 = n / 3;
        let tmp2 = n % 3;
        if tmp2 == 0 { return 3_i32.pow( tmp1 as u32 ); }
        if tmp2 == 1 { return 3_i32.pow(( tmp1 - 1 ) as u32 ) * 4;}
        return 3_i32.pow( tmp1 as u32 ) * 2;
    }
}