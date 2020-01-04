struct Solution;
//TODO improve
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn sub_combination_sum(candidates: &Vec<i32>, sub_target: i32) -> Option<Vec<Vec<i32>>> {
            let mut res = vec![];
            for candidate in candidates {
                //println!("target:{:?}, candidate:{:?}", sub_target, candidate );
                if *candidate == sub_target {
                    //println!("\tpush candidate {:?}", candidate);
                    res.push(vec![*candidate]);
                } else if *candidate < sub_target {
                    //println!("\trecusrive candidate {:?}", candidate);
                    match sub_combination_sum(candidates, sub_target - *candidate) {
                        Some(sub_res) => {
                            for mut s_res in sub_res {
                                s_res.push(*candidate);
                                //println!("found:{:?}", s_res);
                                res.push(s_res);
                            }
                        },
                        None => {continue;},
                    }
                }
                //println!("\tcandidate {:?} is not suitable for target {:?},res:{:?}", candidate, sub_target, res);
            }
            if res.len() == 0 {
                return None;
            } else {
                Some(res)
            }
        }
        match sub_combination_sum(&candidates, target) {
            Some(res_tmp) => {
                use std::collections::HashSet;
                let mut set = HashSet::new();
                for mut t in res_tmp {
                    t.sort();
                    set.insert(t.clone());
                }
                let mut res = vec![];
                for t in set {
                    res.push(t.clone());
                }
                res
            },
            None => {
                vec![]
            },
        }
    }
}