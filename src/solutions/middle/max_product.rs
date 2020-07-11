struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut res = 0_i32;
        let v_words: Vec<Vec<char>> = words.iter().map(|s| s.chars().collect()).collect();
        let check_have = |s1: &Vec<char>, s2: &Vec<char>| -> bool {
            for c in s1 {
                if s2.contains(c) {
                    //println!("s1 and s2 have common char {:?}", c);
                    return false;
                }
            }
            //println!("s1 and s2 check ok");
            return true;
        };
        let mut i = 0_usize;
        let mut j = i + 1;
        loop {
            match v_words.get(i) {
                Some(s1) => {
                    while let Some(s2) = v_words.get(j) {
                        j += 1;
                        //println!("check: s1:{:?}, s2:{:?}", s1, s2);
                        if !check_have(s1, s2) {
                            continue;
                        } else {
                            let tmp = (s1.len() * s2.len()) as i32;
                            //println!("tmp={:?}", tmp);
                            if tmp > res {
                                res = tmp;
                            }
                        }
                    }
                }
                None => {
                    break;
                }
            }
            i += 1;
            j = i + 1;
        }
        res
    }
}
