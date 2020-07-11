impl Solution {
    pub fn max_profit(prices1: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max = 0;
        let mut prices = prices1.clone();
        if prices.len() == 0{
            return 0;
        }
        for i in 0..prices.len()-1{
            prices[i] = prices[i+1] - prices[i];
        }
        for i in 0..prices.len()-1{
            sum = sum + prices[i];
            if sum < 0 {
                sum = 0;
            }
            if sum > max {
                max = sum;
            }
        }
        max
    }
}