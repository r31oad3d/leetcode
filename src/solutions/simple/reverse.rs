struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut res: i64 = 0;
        let result = match x {
            x if x == 0 => 0,
            mut x => loop {
                res = res * 10 + x as i64 % 10;
                x = x / 10;
                if x == 0 {
                    break res;
                }
            },
        };
        if result > i32::max_value() as i64 || result < i32::min_value() as i64 {
            0 as i32
        } else {
            result as i32
        }
        //应该还有更好的rust的方式去判断溢出
    }
}
