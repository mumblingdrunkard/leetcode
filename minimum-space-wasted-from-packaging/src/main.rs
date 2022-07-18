struct Solution {}

impl Solution {
    pub fn min_wasted_space(packages: Vec<i32>, boxes: Vec<Vec<i32>>) -> i32 {
        const M: i64 = 1000000007;
        // allow mutation
        let (mut packages, mut boxes) = (packages, boxes);
        // sort the packages
        packages.sort();
        // calculate the cumulative sum
        let cumulative_size: Vec<_> = [0]
            .iter()
            .chain(packages.iter())
            .scan(0i64, |total, &size| {
                *total = *total + size as i64;
                Some(*total)
            })
            .collect();

        for supplier in boxes.iter_mut() {
            supplier.sort();
        }

        (boxes
            .iter()
            .filter(|supplier| supplier.last().unwrap() >= packages.last().unwrap())
            .map(|supplier| {
                supplier
                    .iter()
                    .scan(0, |split, &box_size| {
                        let new_split =
                            packages.partition_point(|&package_size| package_size <= box_size);
                        let n_packages = new_split - *split;
                        let total_space = n_packages as i64 * box_size as i64;
                        let wasted_space =
                            total_space - (cumulative_size[new_split] - cumulative_size[*split]);
                        *split = new_split;

                        Some(wasted_space)
                    })
                    .fold(0, |sum, wasted_space| sum + wasted_space)
            })
            .min()
            .unwrap_or(-1)
            % M) as i32
    }
}

fn main() {
    let mut packages = vec![0; 100000];
    packages
        .iter_mut()
        .enumerate()
        .for_each(|(i, v)| *v = i as i32 + 1);

    let mut suppliers = vec![vec![0; 2]; 50000];
    suppliers
        .iter_mut()
        .enumerate()
        .for_each(|(i, v)| *v = vec![i as i32 + 50000, 100000]);

    for v in packages.iter().take(5) {
        println!("{}", v);
    }

    println!("{}", Solution::min_wasted_space(packages, suppliers));
}
