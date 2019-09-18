struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        // (40,)41,[91,]93,{123,}125
        if s.len()%2 != 0 {
            return false;
        }
        let mut stack = Vec::<&u8>::new();
        for r#char in s.as_bytes() {
            if stack.len() == 0 {
                stack.push(r#char);
            } else {
                match stack.last() {
                    Some(&c) => {
                        match c {
                            40 => {
                                if r#char == &41 {
                                    let _ = stack.pop();
                                } else {
                                    stack.push(&r#char);
                                }
                            },
                            91 => {
                                if r#char == &93 {
                                    let _ = stack.pop();
                                } else {
                                    stack.push(&r#char);
                                }
                            },
                            123 => {
                                if r#char == &125 {
                                    let _ = stack.pop();
                                } else {
                                    stack.push(&r#char);
                                }
                            },
                            _ => {
                                stack.push(&r#char);
                            },
                        }
                    },
                    None => {},
                }
            }
        }
        stack.len() == 0
    }
}