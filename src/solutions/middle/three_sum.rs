use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = vec![];
        let (mut i, mut j, mut k) = (0, 1, 2);
        let (mut i_tmp, mut j_tmp, mut k_tmp) = (0, 0, 0);
        loop {
            println!("threesum: {},{},{}", i, j, k);
            if *nums.get(i).unwrap() >= 0 {
                break;
            }
            if nums.len() - i == 3 {
                break;
            }
            i_tmp = *nums.get(i).unwrap();
            j_tmp = *nums.get(j).unwrap();
            k_tmp = 0 - i_tmp - j_tmp;
            println!("threesum tmp: {},{},{}", i_tmp, j_tmp, k_tmp);
            for index in k..nums.len() {
                if k_tmp == *nums.get(index).unwrap() {
                    let mut v_tmp = Vec::<i32>::new();
                    v_tmp.push(i_tmp);
                    v_tmp.push(j_tmp);
                    v_tmp.push(k_tmp);
                    res.push(v_tmp);
                    break;
                }
            }
            let mut flag = false;
            for index in i..nums.len() {
                println!("threesum index: {}", index);
                println!(
                    "threesum i_tmp: {}, j_tmp: {}, i {}, j {}",
                    i_tmp,
                    j_tmp,
                    nums.get(index).unwrap(),
                    nums.get(index + 1).unwrap()
                );
                if nums.get(index).unwrap() != &i_tmp {
                    i = i + index;
                    flag = true;
                    break;
                }
            }
            if !flag {
                i = i + 1;
            }
            j = i + 1;
            k = j + 1;
        }
        res
    }
}

//timeout
struct Solution1;
impl Solution1 {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set = HashSet::new();
        let mut res = vec![];
        nums.sort();
        let (mut start, mut end) = (0, nums.len() - 1);

        while start < nums.len() {
            //println!("start: {:?}",nums[start]);
            if nums[start] > 0 {
                break;
            }

            while end > start + 1 {
                //println!("end: {:?}",nums[end]);
                let v_start = nums[start];
                let v_end = nums[end];
                if v_end < 0 {
                    break;
                }
                if set.contains(&vec![v_start, v_end, 0 - v_start - v_end]) {
                    end -= 1;
                    continue;
                }
                let mut index = start + 1;
                while index < end {
                    let v_index = nums[index];
                    if v_start + v_end + v_index == 0 {
                        set.insert(vec![v_start, v_end, v_index]);
                        break;
                    }
                    index += 1;
                }
                end -= 1;
            }
            start += 1;
            end = nums.len() - 1;
        }
        for x in set.iter() {
            res.push(x.clone());
        }
        res
    }
}

pub fn test() {
    let v = vec![-1, 0, 1, 2, -1, -4];
    println!("threesnum : {:?}", Solution::three_sum(v));
}
