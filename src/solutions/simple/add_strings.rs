struct Solution;


impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut res = "".to_string();
        let size = if num1.len() >= num2.len() {
            num1.len()
        } else {
            num2.len()
        };
        let compute = |n1: i32, n2: i32, flag| -> (i32, i32) {
            let tmp = n1 + n2 + flag;
            if tmp >= 10 {
                (tmp - 10, 1)
            } else {
                (tmp, 0)
            }
        };
        let transform_char_i32 = |c: char| -> i32 {
            match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => {panic!("wtf?")},
            }
        };
        let transform_i32_char = |i: i32| -> char {
            match i {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                _ => {panic!("wtf?")},
            }
        };
        let v_num1: Vec<char> = num1.chars().rev().collect();
        let v_num2: Vec<char> = num2.chars().rev().collect();
        let mut i = 0_usize;
        let mut flag = 0;
        while i < size {
            match (v_num1.get(i), v_num2.get(i)) {
                (Some(n1), Some(n2)) => {
                    let (_res, _flag) =
                        compute(transform_char_i32(*n1), transform_char_i32(*n2), flag);
                    flag = _flag;
                    res.insert(0, transform_i32_char(_res));
                }
                (None, Some(n)) | (Some(n), None) => {
                    let (_res, _flag) = compute(transform_char_i32(*n), 0, flag);
                    flag = _flag;
                    res.insert(0, transform_i32_char(_res));
                }
                _ => {
                    panic!("wtf?")
                }
            }
            i += 1;
        }
        if flag == 1 {
            res.insert(0, '1');
        }
        res
    }
}