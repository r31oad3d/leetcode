struct Solution;


impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::BTreeMap;
        let mut nums = nums;
        nums.sort();
        let mut map = BTreeMap::<i32,(i32,i32)>::new();
        for i in nums {
            let ref mut value = map.entry(i).or_insert((i,0));
            value.1 += 1;
        }
        //println!("map:{:?}", map);
        let mut res1: Vec<(i32,i32)> = map.values().cloned().collect();
        res1.sort_by(|item1, item2| item1.1.partial_cmp(&item2.1).unwrap());
        res1.reverse();
        //println!("res1:{:?}", res1);
        let mut res2 = &res1[0..(k as usize)].to_vec();
        //println!("res2:{:?}", res2);
        let mut res3 = Vec::<i32>::with_capacity(res2.len());
        for (i,n) in res2.iter() {
            res3.push(i.clone());
        }
        // println!("{:?}", res1);
        // vec![]
        //println!("res3:{:?}", res3);
        res3
    }
}