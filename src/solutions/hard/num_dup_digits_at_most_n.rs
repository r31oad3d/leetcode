struct Solution;

impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        // 排列组合
        // 从两位数考虑，88，就变成了两个数字 8和8 十位1-8与个位1-8的排列组合中两个位相等的数量，就是1
        // 从三位数考虑，123，就变成了两位数最大数99的结果，加上100-123的结果，也就是1和2和3在各自位上的结果加上前面的99的结果
        // 以此类推，四位数 五位数等
        //   1 ->  0
        //   2 ->  0
        //  11 ->  1 -> 11
        //  21 ->  1 -> 11
        //  22 ->  2 -> 11  22
        //  31 ->  2 -> 11  22
        //  32 ->  2 -> 11  22
        //  33 ->  3 -> 11  22  33
        //  41 ->  3 -> 11  22  33
        //  42 ->  3 -> 11  22  33
        //  43 ->  3 -> 11  22  33
        //  44 ->  4 -> 11  22  33  44
        //  55 ->  11  22  33  44  55
        //  66
        //  77
        //  88
        //  99 ->  9 -> 11  22  33  44  55  66  77  88  99
        // 100 -> 10 -> 11  22  33  44  55  66  77  88  99
        //              100
        // 101 -> 11
        //  1-9
        //  10-99  -> 11  22  33  44  55  66  77  88  99
        //  100-101 ->      100 101
        // 140 -> 25 -> 11  22  33  44  55  66  77  88  99
        // 100 - 140 -> 100 101 110 111 112 113 114 115 116 117 118 119 121 122 131 133
        //
        // 240 -> 53 -> 11  22  33  44  55  66  77  88  99
        //              100 101 110 111 112 113 114 115 116 117 118 119 121 122 131 133 141 144 151 155 161 166 171 177 181 188 191 199
        //              200 202 211 212 220 221 222 223 224 225 226 227 228 229 232 233
        //
        // 999 -> 261-> 11  22  33  44  55  66  77  88  99
        // 100 - 199    100 101 110 111 112 113 114 115 116 117 118 119 121 122 131 133 141 144 151 155 161 166 171 177 181 188 191 199
        // 200 - 299    200 202 211 212 220 221 222 223 224 225 226 227 228 229 232 233 242 244 252 255 262 266 272 277 282 288 292 299
        // 300 - 399    300 303 311 313 322 323 331 332 333 334 335 336 337 338 339 343 344 352 353 355 362 366 373 377 383 388 393 399
        //   (9+252)    ...
        // 1000-> 262-> ...
        //  (9+252+1)   ...
        // 1000-1999
        // 2000-2999
        // 3000-3999
        // 4000-4999
        // 15687
        //      --> {1-9999} + {10000-15687}
        //      --> {1-999} + {1000-9999} + {10000-15687}
        //      --> {1-99} + {100-999} + {1000-9999} + {10000-15687}
        //      --> {1-9} + {10-99} + {100-999} + {1000-9999} + {10000-15687}
        //      -->  1位数 + 2位数 + 3位数 + 4位数 + tail
        // n位数，排列组合至少一个重复数字
        let mut res: i32 = 0;
        let length = is_sized_of(n);
        let min_value_of_n_size = 10_i32.pow(length as u32 - 1);
        let remain = n - min_value_of_n_size;
        if length == 1 {
            println!("n{}, got:{}", n, n_size_num_dup_at_most(1, n));
            return n_size_num_dup_at_most(1, n);
        } else if length == 2 {
            println!("n{}, got:{}", n, n_size_num_dup_at_most(2, n));
            return n_size_num_dup_at_most(2, n);
        } else {
            for cursor in 1..=length - 1 {
                if cursor == 2 {
                    println!("{}position,  n{}, got:{}", cursor, n, n_size_num_dup_at_most(cursor, 99));
                    res += n_size_num_dup_at_most(cursor, 99);
                } else {
                    println!("{}position,  n{}, got:{}", cursor, n, n_size_num_dup_at_most(cursor, n));
                    res += n_size_num_dup_at_most(cursor, n);
                }
            }
        }
        if remain >= 0 {
            res += from_n_sized_num_starter_to_num_dup_at_most_adverse( min_value_of_n_size, remain, n);
        } else {
            panic!("wtf?!")
        }
        res
    }
}

const MAX_VALUE_N: i32 = 9;

// 2147483647 位数-1
fn is_sized_of(mut i: i32) -> i32 {
    let mut count = 0;
    while i != 0 {
        count += 1;
        i /= 10;
    }
    count
}


fn n_size_num_dup_at_most(n: i32, i: i32) -> i32 {
    //n : 几位数，i: 具体数字
    match n {
        1 => { 0 }
        2 => {
            match i {
                10 => { 0 }
                11..=21 => { 1 }
                22..=32 => { 2 }
                33..=43 => { 3 }
                44..=54 => { 4 }
                55..=65 => { 5 }
                66..=76 => { 6 }
                77..=87 => { 7 }
                88..=98 => { 8 }
                99 => { 9 }
                _ => { 0 }
            }
        }
        n => {
            // 三位数以及以上
            let max_n_sized_num = 10_i32.pow(n as u32 - 1) * 9;
            let mut res_adverse = 1;
            // 排列组合
            // 三位数 9 * 8
            // 四位数 9 * 8 * 7
            for num in 2..=n {
                res_adverse *= (10 - num + 1);
            }
            res_adverse *= 9;
            max_n_sized_num - res_adverse
        }
    }
}


fn from_n_sized_num_starter_to_num_dup_at_most_adverse(min_value_of_n_size: i32, remain: i32, n: i32 ) -> i32 {
    //tail
    // 982 -> 1-9 + 10-99 + 100-982
    // 100-982
    // 105 -> 1-9 + 10-99 + 100-105
    // 100-105

    if remain == 0 {
        return 1;
    } else {
        println!("tail calculate!, remain is {}, tail is {}-{}", remain,  min_value_of_n_size, n);
        let mut v_num:Vec<i32> = Vec::new();
        let mut i = n;
        while i != 0 {
            v_num.push(i%10);
            i /= 10;
        }
        v_num.reverse();
        let max_n_sized_num = 1;
        let mut num_at_previous_position = 0;
        for num_at_position in v_num{
            let mut tmp = num_at_position < num_at_previous_position;
            //max_n_sized_num *= num_at_position < num_at_previous_position
        }
    }

    2
}

pub fn test() {
    println!("77!!! {}", Solution::num_dup_digits_at_most_n(77));
    println!("99!!! {}", Solution::num_dup_digits_at_most_n(99));
    println!("100!!! {}", Solution::num_dup_digits_at_most_n(100));
    println!("101!!! {}", Solution::num_dup_digits_at_most_n(101));
    println!("999!!! {}", Solution::num_dup_digits_at_most_n(999));
    println!("1000!!! {}", Solution::num_dup_digits_at_most_n(1000));
    println!("1001!!! {}", Solution::num_dup_digits_at_most_n(1001));

    let mut v_num:Vec<i32> = Vec::new();
    let mut i = 987;
    while i != 0 {
        v_num.push(i%10);
        i /= 10;
    }
    v_num.reverse();
    println!("{:?}", v_num);
}