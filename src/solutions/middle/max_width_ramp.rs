struct Solution;

impl Solution {
    pub fn max_width_ramp(a: Vec<i32>) -> i32 {
        let mut a_ =
            a.iter().enumerate().map(|(i, _)| i).collect::<Vec<usize>>();
        a_.sort_unstable_by(|i, j| a[*i].cmp(&a[*j]));
        let mut res = 0_i32;
        let mut m = a.len() as i32;
        a_.iter().for_each(|i| {
            let i_ = *i as i32;
            res = std::cmp::max(res, i_ - m);
            m = std::cmp::min(m, i_);
        });
        res
    }
}
