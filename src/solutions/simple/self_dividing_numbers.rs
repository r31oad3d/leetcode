struct Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res = vec![];
        for mut num in left..=right {
            if num < 10 {
                res.push(num);
                continue;
            }
            let mut flag = true;
            let mut tmp1 = num;
            println!("handling {}", tmp1);
            while tmp1 > 0 {
                let tmp2 = tmp1 % 10;
                tmp1 = tmp1 / 10;
                println!("!!!!handling {}", tmp2);
                if tmp2 == 0 || num % tmp2 != 0 {
                    flag = false;
                }
            }
            if flag {
                res.push(num);
            }
        }
        res
    }
}


pub fn test() {
    println!("{:?}", Solution::self_dividing_numbers(1, 22));
}