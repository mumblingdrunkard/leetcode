struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |res, n| res^n)
    }
}

fn main() {
    println!("{}", Solution::single_number(vec![4, 1, 2, 1, 2]));
}
