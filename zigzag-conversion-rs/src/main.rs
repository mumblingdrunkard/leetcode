struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let num_rows = num_rows as usize;
        let mut res = Vec::with_capacity(s.len());

        {
            let chars = s.as_bytes();
            let step = num_rows * 2 - 2;

            let mut left = 0;
            while left < chars.len() {
                res.push(chars[left]);
                left += step;
            }

            for r in 1..(num_rows - 1) {
                let mut left = r;
                let mut right = step - r;
                while left < chars.len() {
                    res.push(chars[left]);
                    if right < chars.len() {
                        res.push(chars[right]);
                    }
                    left += step;
                    right += step;
                }
            }

            left = num_rows - 1;
            while left < chars.len() {
                res.push(chars[left]);
                left += step;
            }
        }

        String::from_utf8(res).unwrap()
    }
}

fn main() {
    println!("{}", Solution::convert("PAYPALISHIRING".to_string(), 3));
}
