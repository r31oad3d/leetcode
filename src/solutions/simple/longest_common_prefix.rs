struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut strs: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();
        let mut index_in_str = 0_usize;
        let mut res = String::new();
        'outer: loop {
            let mut cur_loop_c: char = ' ';
            for i in 0..strs.len() {
                let temp = strs.get(i).unwrap();
                match temp.get(index_in_str) {
                    Some(c_temp) => {
                        if i == 0 {
                            cur_loop_c = *c_temp as char;
                            if i == strs.len() - 1 {
                                res.push(cur_loop_c);
                                index_in_str += 1;
                                continue 'outer;
                            }
                        } else {
                            if cur_loop_c == *c_temp as char {
                                if i == strs.len() - 1 {
                                    res.push(cur_loop_c);
                                    index_in_str += 1;
                                    continue 'outer;
                                }
                            } else {
                                break 'outer;
                            }
                        }
                    }
                    None => {
                        break 'outer;
                    }
                }
            }
            break;
        }
        res
    }
}
