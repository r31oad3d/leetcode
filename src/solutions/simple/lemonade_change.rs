struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut doller_pool_5 = 0_i32;
        let mut doller_pool_10 = 0_i32;

        for i in bills {
            println!(
                "we have {} 5_doller, {} 10_doller, incoming {}",
                doller_pool_5, doller_pool_10, i
            );
            match i {
                5 => {
                    doller_pool_5 += 1;
                }
                10 => {
                    if doller_pool_5 == 0 {
                        return false;
                    } else {
                        doller_pool_5 -= 1;
                        doller_pool_10 += 1;
                        continue;
                    }
                }
                20 => {
                    if doller_pool_10 == 0 {
                        if doller_pool_5 < 3 {
                            return false;
                        } else {
                            doller_pool_5 -= 3;
                        }
                    } else {
                        doller_pool_10 -= 1;
                        if doller_pool_5 < 1 {
                            return false;
                        } else {
                            doller_pool_5 -= 1;
                        }
                    }
                }
                _ => panic!("wtf?!"),
            }
        }
        return true;
    }
}
