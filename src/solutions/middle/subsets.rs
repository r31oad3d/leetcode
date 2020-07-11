struct Solution;
/*
给定一组不含重复元素的整数数组 nums，返回该数组所有可能的子集（幂集）。

说明：解集不能包含重复的子集。

示例:

输入: nums = [1,2,3]
输出:
[
  [3],
[1],
[2],
[1,2,3],
[1,3],
[2,3],
[1,2],
[]
]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/subsets
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */
//not done
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            return vec![vec![nums[0]]];
        } else {
            let temp_value = nums[0];
            let mut temp_res = Solution::subsets(nums[1..].to_vec());
            let res  = temp_res.iter().map(| v| {
                let mut v_temp = v.clone();
                v_temp.push(temp_value);
                return v_temp;
            }).collect::<Vec<Vec<i32>>>();
            return res ;
        }
    }
}

pub fn test() {
    let v = vec![1, 2, 3];
    println!("{:?}", Solution::subsets(v));
}
