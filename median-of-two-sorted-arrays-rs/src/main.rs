#[allow(unused)]
struct Solution {}

#[allow(unused)]
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        use std::cmp;

        let (a, b) = match nums1.len() < nums2.len() {
            true => (nums1, nums2),
            _ => (nums2, nums1),
        };

        let length = a.len() + b.len();
        let half = length / 2;

        // start with all elements from a (the shorter array)
        let mut i = a.len();
        let mut step = (half / 2).clamp(1, usize::MAX);

        loop {
            // supplement with elements from b (the longer array)
            let mut j = half - i;

            let (mut al, mut ar, mut bl, mut br) = (i32::MIN, i32::MAX, i32::MIN, i32::MAX);
            (i > 0).then(|| al = a[i - 1]);
            (i < a.len()).then(|| ar = a[i]);
            (j > 0).then(|| bl = b[j - 1]);
            (j < b.len()).then(|| br = b[j]);

            println!("(i, j) = {:?}", (i, j));
            println!("(al, ar, bl, br) = {:?}", (al, ar, bl, br));

            if al <= br && bl <= ar {
                return if length % 2 == 0 {
                    (cmp::max(al, bl) + cmp::min(ar, br)) as f64 / 2.0
                } else {
                    cmp::min(ar, br) as f64
                };
            } else if al < br {
                i = (i + step).clamp(0, a.len());
            } else {
                i = (i - step).clamp(0, a.len());
            }

            step = (step / 2).clamp(1, usize::MAX);
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![], vec![1])
    );
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5, 6, 7])
    );
}
