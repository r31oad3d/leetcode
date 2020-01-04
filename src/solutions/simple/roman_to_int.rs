struct Solution;


impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        //  字符          数值
        //  I             1
        //  V             5
        //  X             10
        //  L             50
        //  C             100
        //  D             500
        //  M             1000

        let mut unit = 0_i32;
        let mut decade = 0_i32;
        let mut hundred = 0_i32;
        let mut thousand = 0_i32;

        //let map = ((('I', 1), ('V', 5)), (('X', 10), ('L', 50)), (('C', 100),('D', 500)),(('M',100),));
        let mut previous: char = 0 as char;
        for c in s.chars().rev() {
            match c {
                'M' => {
                    thousand += 1000;
                    previous = 'M';
                },
                'D' => {
                    hundred += 500;
                    previous = 'D';
                },
                'C' => {
                    if let 'C' = previous {
                        hundred += 100;
                    } else if let 'M' = previous {
                        hundred = 900;
                        thousand = 0;
                    } else if let 'D' = previous {
                        hundred = 400;
                    } else {
                        hundred += 100;
                    }
                    previous = 'C';
                },
                'L' => {
                    decade += 50;
                    previous = 'L';
                },
                'X' => {
                    if let 'X' = previous {
                        decade += 10;
                    } else if let 'C' = previous {
                        decade = 90;
                        hundred = 0;
                    } else if let 'L' = previous {
                        decade = 40;
                    } else {
                        decade += 10;
                    }
                    previous = 'X';
                },
                'V' => {
                    unit += 5;
                    previous = 'V';
                },
                'I' => {
                    if let 'I' = previous {
                        unit += 1;
                    } else if let 'X' = previous {
                        unit = 9;
                        decade = 0;
                    } else if let 'V' = previous {
                        unit = 4;
                    } else {
                        unit += 1;
                    }
                    previous = 'I';
                },
                _ => {}
            }
        }


        thousand + hundred + decade + unit
    }
}