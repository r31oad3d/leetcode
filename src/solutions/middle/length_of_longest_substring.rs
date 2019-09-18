struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
//        let v_s = Vec::from(s.clone());
//        if s.len() == 0 {
//            return 0;
//        }
//        if s.trim().len() == 0 {
//            return 0;
//        }
//        if s.len() == 1 {
//            return 1;
//        }
//        let (mut i, mut j, mut length) = (0, 1, 1);
//        loop {
//            let mut len = 0;
//            let (mut cur_i, mut cur_j) = (&v_s[i], &v_s[j]);
//            if &s[i..j].contains(String::from(&v_s[j + 1] as char)) {
//                len = j - i + 1;
//                i += 1;
//                j = i + 1;
//                if len > length {
//                    length = len;
//                }
//            } else {
//                j += 1;
//            }
//
//            if j >= s.len() - 1 {
//                len = j - i + 1;
//                if len > length {
//                    length = len;
//                }
//                break;
//            }
//            if i >= s.len() - 1 {
//                len = j - i + 1;
//                if len > length {
//                    length = len;
//                }
//                break;
//            }
//        }
//        length as i32
        5
    }
}

fn test() {
    //println!(Solution::length_of_longest_substring("abcabcabcacaaw".to_string()));
}