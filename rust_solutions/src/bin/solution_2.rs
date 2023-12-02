use itertools::Itertools;
use std::cmp;

fn main() {
    let result: u32 = std::fs::read_to_string("input2.txt")
        .unwrap()
        .split('\n')
        .map(|line| {
            let parts = line.split(':').collect_vec();

            if parts.len() < 2 {
                return 0;
            }

            let highest_colors = parts[1]
                .split(';')
                .fold((0, 0, 0), |maximums, curr_subset| {
                    let curr_set = curr_subset.split(',').fold((0, 0, 0), |maximums, curr| {
                        let val_keys = curr.trim().split(' ').collect_vec();
                        let val = val_keys[0].parse::<u32>().unwrap();

                        match val_keys[1] {
                            "red" => (cmp::max(maximums.0, val), maximums.1, maximums.2),
                            "green" => (maximums.0, cmp::max(maximums.1, val), maximums.2),
                            _ => (maximums.0, maximums.1, cmp::max(maximums.2, val)),
                        }
                    });

                    (
                        cmp::max(curr_set.0, maximums.0),
                        cmp::max(curr_set.1, maximums.1),
                        cmp::max(curr_set.2, maximums.2),
                    )
                });

            // Part: 1
            // if highest_colors.0 <= 12 && highest_colors.1 <= 13 && highest_colors.2 <= 14 {
            //     parts[0].split(' ').last().unwrap().parse::<u32>().unwrap()
            // } else {
            //     0
            // }

            // Part: 2
            highest_colors.0 * highest_colors.1 * highest_colors.2
        })
        .sum();

    println!("{:?}", result);
}
