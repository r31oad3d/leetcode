struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        fn sub_(mut chars: Vec<char>) -> Vec<String> {
            let mut res: Vec<String> = vec![];
            if let Some(c) = chars.pop() {
                // println!("c={:?}, chars={:?}", c, chars);
                sub_(chars).into_iter().for_each(|s| {
                    if c.is_alphabetic() {
                        res.push(
                            s.clone() + c.to_lowercase().to_string().as_str(),
                        );
                        res.push(s + c.to_uppercase().to_string().as_str());
                    } else {
                        res.push(s + c.to_string().as_str());
                    }
                });
            } else {
                res.push(String::new());
            }
            res
        }
        let a = s.chars().collect::<Vec<char>>();
        // println!("{:?}", a);
        sub_(a)
    }
}

#[test]
fn test_it() {
    // let a : Vec<&str> = "a1b2".split(char::is_numeric).collect::<Vec<&str>>();
    // println!("{:?}", a);
    //
    // println!("{}", '1'.to_uppercase());

    println!("{:?}", Solution::letter_case_permutation("a1b1".to_owned()));
}
