use leetcode::*;

fn main() {
    let nums = [4, 1, 7, 4, 6].to_vec();
    let num_bottles = 9;
    let num_exchange = 3;
    println!(
        "{}",
        medium::Solution::num_water_bottles(num_bottles, num_exchange)
    );
}
