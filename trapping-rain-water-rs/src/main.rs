struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        use std::cmp;

        // tallest_rl[n + 1] gives the tallest bar seen after height[n]
        let tallest_rl: Vec<_> = height
            .iter()
            .rev()
            .scan(0, |tallest, &h| {
                *tallest = cmp::max(*tallest, h);
                Some(*tallest)
            })
            .collect::<Vec<i32>>()
            .into_iter()
            .rev()
            .collect();

        let mut h = 0;
        let mut i = 0;
        let mut s = 0;
        while i < height.len() {
            while i < height.len() - 1 && height[i] <= h && h <= tallest_rl[i + 1] {
                s += h - height[i];
                i += 1;
            }

            i += 1;
            while i < height.len() - 1 && height[i] > tallest_rl[i + 1] {
                i += 1;
            }
            i -= 1;

            if i < height.len() - 1 {
                h = cmp::min(height[i], tallest_rl[i + 1]);
            }

            i += 1;
        }

        s
    }
}

fn main() {
    println!("{}", Solution::trap(vec![4, 2, 0, 3, 2, 5]));

    println!("{}", Solution::trap(vec![4, 2, 3]));

    println!("{}", Solution::trap(vec![5, 4, 1, 2]));
}
