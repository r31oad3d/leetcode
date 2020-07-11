struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = stones;
        stones.sort();
        loop {
            if stones.len() <= 1 {
                break;
            }
            let i = stones.len() - 1;
            let diff = stones[i] - stones[i - 1];
            stones.pop();
            stones.pop();
            if diff > 0 {
                stones.push(diff);
            }
            stones.sort();
        }
        if stones.len() == 0 {
            return 0;
        }
        return stones[0];
    }
}
