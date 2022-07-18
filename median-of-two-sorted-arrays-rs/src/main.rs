#[allow(unused)]
struct Solution {}

#[allow(unused)]
impl Solution {
    // idea: find splits i and j in nums1 and nums2 such that
    // a) i + j = (nums1.len() + nums2.len()) / 2, and
    // b) nums1[i-1] <= nums2[j] && nums2[j-1] <= nums1[i]
    //
    // These two conditions will find the mid-point of the merged array
    // To achieve the condition of O(log(n + m)) we can binary search for this split point.
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
        let mut step = (half / 2).clamp(1, usize::MAX); // binary search, step can't be less than 1

        loop {
            // supplement with elements from b (the longer array) until we have half the elements
            let mut j = half - i;

            // initialise al, ar, bl, br with default values
            let (mut al, mut ar, mut bl, mut br) = (i32::MIN, i32::MAX, i32::MIN, i32::MAX);
            // fill values if possible
            // condition.then(|| something); is functionally equivalent to
            // condition && { something; true/false };
            (i > 0).then(|| al = a[i - 1]);
            (i < a.len()).then(|| ar = a[i]);
            (j > 0).then(|| bl = b[j - 1]);
            (j < b.len()).then(|| br = b[j]);

            // we found the mid-point
            if al <= br && bl <= ar {
                return if length % 2 == 0 {
                    (cmp::max(al, bl) + cmp::min(ar, br)) as f64 / 2.0
                } else {
                    cmp::min(ar, br) as f64
                };
            } else if bl > ar {
                // split point in a is too far left
                i = (i + step).clamp(0, a.len());
            } else {
                // split point in a is too far right
                i = (i - step).clamp(0, a.len());
            }

            // halve step
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
