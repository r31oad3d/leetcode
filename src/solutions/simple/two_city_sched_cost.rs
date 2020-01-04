struct Solution;

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut costs = costs;
        let mut res = 0_i32;
        costs.sort_by(|v1,v2| (v1[0]-v1[1]).partial_cmp(&(v2[0]-v2[1])).unwrap());
        let mut i = 0;
        let n = costs.len()/2;
        loop {
            if i < n {
                res += costs[i][0];
            } else if i >= n && i < costs.len()  {
                res += costs[i][1];
            } else if i == costs.len() {
                break;
            }
            i += 1;
        };
        res
    }
}