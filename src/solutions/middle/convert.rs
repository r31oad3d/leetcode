struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let ss: Vec<char> = s.chars().collect();
        let mut res = String::new();
        let step = num_rows * 2 - 2;
        let mut index = 0;
        let mut add = 0;
        for n in 0..num_rows {
            index = n;
            add = n * 2;
            while let Some(&ch) = ss.get(index as usize) {
                res.push(ch);
                add = step - add;
                index += if n == 0 || n == num_rows - 1 {
                    step
                } else {
                    add
                };
            }
        }
        res
    }
}
