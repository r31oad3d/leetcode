struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut another_nums = nums.clone();
        another_nums.sort();
        another_nums.dedup();
        //println!("{:?}", nums);
        //println!("{:?}", another_nums);
        nums.len() != another_nums.len()
    }
}