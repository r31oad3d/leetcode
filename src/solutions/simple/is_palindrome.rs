struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x {
            x if x < 0 => false,
            x if x == 0 => true,
            x => {
                let mut s = x.to_string();
                let mut v = Vec::from(s.clone());
                v.reverse();
                let ss = String::from_utf8(v).unwrap();
                if ss.eq(&s) {
                    true
                } else {
                    false
                }
            }
        }
    }
}

pub fn test() {

}