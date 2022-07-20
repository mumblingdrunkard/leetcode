struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        { 0..num_rows }
            .scan(vec![1], |current, _| {
                let mut previous = [0]
                    .iter()
                    .chain(current.iter())
                    .zip(current.iter().chain([0].iter()))
                    .map(|(l, r)| l + r)
                    .collect();
                std::mem::swap(&mut previous, current);
                Some(previous)
            })
            .collect()
    }
}

fn main() {
    println!("{:?}", Solution::generate(30));
}
