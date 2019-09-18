

struct Solution;

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
//    4 -> 0b00000000000000000000000000000100 ->  3  -> 4^1
//   16 -> 0b00000000000000000000000000010000 ->  5  -> 4^2
//   64 -> 0b00000000000000000000000001000000 ->  7  -> 4^3
//  256 -> 0b00000000000000000000000100000000 ->  9  -> 4^4
// 1024 -> 0b00000000000000000000010000000000 -> 11  -> 4^5
// 4096 -> 0b00000000000000000001000000000000 -> 13  -> 4^6
//16384 -> 0b00000000000000000100000000000000 -> 15  -> 4^7

//         0b01010101010101010101010101010100
        match num {
            0 => {false},
            1 => {true},
            num if num.abs()&(num.abs()-1) != 0 => {false},
            _ => {num == num.abs()&(0b01010101010101010101010101010100) as i32},
        }
    }
}

pub fn test() {
    println!("bitand: {}={}", 4, Solution::is_power_of_four(4));
    println!("bitand: {}={}", 16, Solution::is_power_of_four(16));
    println!("bitand: {}={}", 64, Solution::is_power_of_four(64));
    println!("bitand: {}={}", 256, Solution::is_power_of_four(256));
    println!("bitand: {}={}", 1024, Solution::is_power_of_four(1024));
    println!("bitand: {}={}", 4096, Solution::is_power_of_four(4096));
    println!("bitand: {}={}", 16384, Solution::is_power_of_four(16384));
    println!("bitand: {}={}", 5, Solution::is_power_of_four(5));
    println!("bitand: {}={}", 8, Solution::is_power_of_four(8));
    println!("bitand: {}={}", -4, Solution::is_power_of_four(-4));
    println!("bitand: {}={}", -64, Solution::is_power_of_four(-64));
    println!("bitand: {}={}", 179, Solution::is_power_of_four(179));
    println!("bitand: {}={}", 13, Solution::is_power_of_four(13));
    println!("bitand: {}={}", 999999999, Solution::is_power_of_four(999999999));
    println!("bitand: {}={}", 0, Solution::is_power_of_four(0));
    println!("bitand: {}={}", 20, Solution::is_power_of_four(20));
    println!("bitand: {}={}", 68, Solution::is_power_of_four(68));
    println!("bitand: {}={}", -68, Solution::is_power_of_four(-68));
}