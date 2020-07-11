struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut max_profit, mut min_price) = (0_i32, i32::max_value());
        for i in prices {
            if i < min_price {
                min_price = i;
            } else if i - min_price > max_profit {
                max_profit = i - min_price;
            }
        }
        max_profit
    }
}
