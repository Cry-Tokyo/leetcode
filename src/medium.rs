pub struct Solution;
impl Solution {
    /// 2221. Find Triangular Sum of an Array
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut buf_nums = nums;
        //let mut buf_nums = nums.iter().peekable();
        while buf_nums.len() > 1 {
            buf_nums = buf_nums.windows(2).map(|f| (f[0] + f[1]) % 10).collect();
            //buf_nums =buf_nums.map(|f|) //windows(2).map(|f| (f[0] + f[1]) % 10).collect();
        }
        buf_nums[0]
        //*buf_nums.nth(0).unwrap()
    }
    /// 1518. Water Bottles
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut num_bottles = num_bottles;
        let mut drunk = 0;
        while num_bottles >= num_exchange {
            num_bottles += 1;
            drunk += num_exchange;
            num_bottles -= num_exchange;
        }
        return drunk + num_bottles;
    }
    /// 3147. Taking Maximum Energy From the Mystic Dungeon
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let mut e = 0;

        energy.iter().step_by(k.unsigned_abs() as usize).map(|wiz|);
    }
}
