struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let tallest_lr: Vec<_> = height
            .iter()
            .scan(0, |tallest, &h| {
                *tallest = std::cmp::max(*tallest, h);
                Some(*tallest)
            })
            .collect();

        let tallest_rl: Vec<_> = height
            .iter()
            .rev()
            .scan(0, |tallest, &h| {
                *tallest = std::cmp::max(*tallest, h);
                Some(*tallest)
            })
            .collect::<Vec<i32>>()
            .into_iter()
            .rev()
            .collect();

        fn max_area_internal(lr: &[i32], rl: &[i32], height: &[i32]) -> i32 {
            let mut current = std::cmp::min(height.first().unwrap(), height.last().unwrap())
                * (height.len() - 1) as i32;

            let first = height.first().unwrap();
            let last = height.last().unwrap();
            let tallest_lr = lr.last().unwrap();
            let tallest_rl = rl.first().unwrap();

            if height.len() == 2 {
                return current;
            }

            if tallest_lr > first && first <= last {
                current =
                    std::cmp::max(current, max_area_internal(&lr[1..], &rl[1..], &height[1..]));
            } else if tallest_rl > last && first > last {
                current = std::cmp::max(
                    current,
                    max_area_internal(
                        &lr[..lr.len() - 1],
                        &rl[..rl.len() - 1],
                        &height[..height.len() - 1],
                    ),
                );
            }

            current
        }

        max_area_internal(&tallest_lr, &tallest_rl, &height)
    }
}

fn main() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}
