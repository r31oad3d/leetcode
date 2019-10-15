struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut front, mut end, mut middle) = (0_usize, nums.len() - 1, 0_usize);
        while middle <= end {
            println!("front:{}, middle:{}, end:{}", front, middle, end);
            let mut middle_value = nums[middle];
            if middle_value == 0 {
                nums.swap(middle, front);
                middle += 1;
                front += 1;
            } else if middle_value == 2 {
                nums.swap(middle, end);
                if end == 0 {
                    break;
                }
                end -= 1;
            } else {
                middle += 1;
            }
        }
    }
}


pub fn test() {
    let mut v = vec![2];
    Solution::sort_colors(&mut v);
    println!("{:?}", v);
}