struct Solution;
impl Solution {
    pub fn my_sqrt_v1(x: i32) -> i32 {
        let mut x = x as f32;
        let mut i: i64 = 0;
        let mut x2: f32 = x * 0.5_f32;
        let mut y: f32 = x;
        let threehalfs = 1.5_f32;

        unsafe {
            let step1 = &mut y as *mut f32;
            let step2 = step1 as *mut i64;
            i = *step2;
            i = 0x5f3759df - (i >> 1);
            //println!("{}", i);
            let step3 = &mut i as *mut i64;
            let step4 = step3 as *mut f32;
            y = *step4;
        }
        y = y * (threehalfs - (x2 * y * y));
        y = y * (threehalfs - (x2 * y * y));
        y = y * (threehalfs - (x2 * y * y));
        println!("div: {}", 1.0_f32 / y as f32);
        (1.0_f32 / y as f32) as i32
    }

    pub fn my_sqrt(x: i32) -> i32 {
        if x < 0 {
            panic!("negtive num is not allowed");
        }
        if x == 0 {
            return 0;
        }
        let mut x = x as f64;
        let mut cur = 1_f64;
        loop {
            let pre = cur;
            cur = (cur + x / cur) / 2_f64;
            println!("{},{}", cur, (cur - pre).abs());
            if (cur - pre).abs() < 1e-8 {
                return cur as i32;
            }
        }
    }
}

pub fn test() {
    println!("{},{}", 2147395599, Solution::my_sqrt(2147395599));
    //1073697800,1073697800
    //536848900,536848900
    //268424450,268424450
    //134212220,134212220
    //67106120,67106104
    //33553076,33553044
    //16776570,16776506
    //8388349,8388221
    //4194302.5,4194046.5
    //2097407.3,2096895.3
    //1049215.5,1048191.75
    //525631.06,523584.44
    //264858.22,260772.84
    //136482.97,128375.25
    //76108.38,60374.586
    //52161.676,23946.707
    //46664.875,5496.801
    //46341.133,323.7422
    //46340,1.1328125
    //46340,0

    //1073697800,1073697799
    //536848901,536848899
    //268424452.5,268424448.5
    //134212230.24999996,134212222.25000004
    //67106123.12499966,67106107.1250003
    //33553077.562497288,33553045.56250237
    //16776570.781228304,16776506.781268984
    //8388349.390451395,8388221.390776909
    //4194302.6939235963,4194046.6965277987
    //2097407.3365452527,2096895.3573783436
    //1049215.5849522296,1048191.7515930231
    //525631.1263027362,523584.4586494934
    //264858.24621515913,260772.8800875771
    //136482.98140666296,128375.26480849617
    //76108.38948026267,60374.59192640029
    //52161.67759231341,23946.711887949263
    //46664.87383988814,5496.803752425272
    //46341.13085071446,323.7429891736829
    //46340.000003008376,1.1308477060811128
    //46339.999989210184,0.000013798191503155977
    //46339.999989210184,0
}
