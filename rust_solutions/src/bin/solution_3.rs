use itertools::Itertools;

// Example part 1, with sum 4361
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..

fn main() {
    let mut grid = std::fs::read_to_string("input3.txt")
        .unwrap()
        .split('\n')
        .map(|line| {
            let mut vec = line.chars().collect::<Vec<char>>();
            // Pad left/right to be able to skip bounds checking
            vec.push('.');
            vec.insert(0, '.');

            vec
        })
        .collect_vec();

    // Remove empty last row
    grid.pop();

    // Pad up/down to be able to skip bounds checking
    grid.insert(0, vec!['.'; grid[0].len()]);
    grid.push(vec!['.'; grid[0].len()]);

    let result: u32 = grid
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| {
            let mut include_curr = false;
            let mut curr_digits: Vec<char> = Vec::new();
            let mut row_values: Vec<u32> = Vec::new();

            for (i, ch) in b.iter().enumerate() {
                if i == 0 {
                    continue;
                }

                if i == b.len() {
                    break;
                }

                match ch {
                    curr_char if curr_char.is_ascii_digit() => {
                        curr_digits.push(*curr_char);
                        if (b[i - 1] != '.' && !b[i - 1].is_ascii_digit())
                            || (b[i + 1] != '.' && !b[i + 1].is_ascii_digit())
                            || (a[i] != '.' && !a[i].is_ascii_digit())
                            || (a[i - 1] != '.' && !a[i - 1].is_ascii_digit())
                            || (a[i + 1] != '.' && !a[i + 1].is_ascii_digit())
                            || (c[i] != '.' && !c[i].is_ascii_digit())
                            || (c[i - 1] != '.' && !c[i - 1].is_ascii_digit())
                            || (c[i + 1] != '.' && !c[i + 1].is_ascii_digit())
                        {
                            include_curr = true;
                        }
                    }
                    _ => {
                        if include_curr && !curr_digits.is_empty() {
                            let num_string = curr_digits.iter().collect::<String>();
                            row_values.push(num_string.parse::<u32>().unwrap());
                            include_curr = false;
                        }

                        curr_digits.clear();
                    }
                }
            }

            row_values.iter().sum::<u32>()
        })
        .sum();

    println!("{:?}", result);
}
