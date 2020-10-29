struct Solution;

impl Solution {
    pub fn find_repeat_numberfind_repeat_number(nums: Vec<i32>) -> i32 {
        // use std::collections::HashSet;
        // let mut set = HashSet::<i32>::new();
        // let mut repeat = -1;
        // for n in nums {
        //     if !set.insert(n) {
        //         repeat = n;
        //         break;
        //     }
        // }
        // repeat

        let mut temp = -1_i32;
        let mut nums = nums;
        let mut i = 0_usize;
        while i < nums.len() {
            while nums[i] as usize != i {
                if nums[i] == nums[nums[i] as usize] {
                    return nums[i];
                }
                temp = nums[i];
                nums.swap(i, temp as usize);
            }
            i += 1;
        }
        -1
    }
}

#[test]
fn test_it() {
    println!(
        "{}",
        Solution::find_repeat_numberfind_repeat_number(vec![
            2, 3, 1, 0, 2, 5, 3
        ])
    );
}
