struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        //  字符          数值
        //  I             1
        //  V             5
        //  X             10
        //  L             50
        //  C             100
        //  D             500
        //  M             1000
        let map = [
            "I", "II", "III", "VI", "V", "IV", "IIV", "IIIV", "XI", "X", "XX",
            "XXX", "LX", "L", "XL", "XXL", "XXXL", "CX", "C", "CC", "CCC",
            "DC", "D", "CD", "CCD", "CCCD", "MC", "M", "MM", "MMM",
        ];
        let mut remainder: usize = 0;
        let mut temp = String::new();
        let mut result = String::new();
        let mut num = num as usize;
        let mut factor = 0;
        while num > 0 {
            remainder = num % 10;
            let index = if remainder == 0 {
                None
            } else {
                Some(factor * 9 + (remainder - 1))
            };
            match index {
                None => {}
                Some(index) => {
                    temp.push_str(map[index]);
                }
            };
            factor += 1;
            num = num / 10;
        }
        for c in temp.chars().rev() {
            result.push(c);
        } //没有一键反转字符串？
        result
    }
}
