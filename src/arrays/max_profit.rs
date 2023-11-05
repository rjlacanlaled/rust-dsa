pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = prices[0];
    let mut current_max_profit = prices[0];

    for i in 1..prices.len() {
        current_max_profit = std::cmp::max(0, current_max_profit + prices[i] - prices[i - 1]);
        max_profit = std::cmp::max(max_profit, current_max_profit);
    }

    println!("max_profit {:?}", max_profit);
    max_profit
}
