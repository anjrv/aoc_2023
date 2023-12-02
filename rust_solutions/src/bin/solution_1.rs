fn main() {
    let result: u32 = std::fs::read_to_string("input1.txt")
        .unwrap()
        .split('\n')
        .map(|line| {
            // Inefficient replace hack for part 2 to allow partial matches like eightwone
            line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .chars()
                .filter(|x| x.is_ascii_digit())
                .collect::<Vec<char>>()
                .into_iter()
        })
        .map(|mut digits| {
            let first = digits.next();

            if first.is_none() {
                return 0;
            }

            format!(
                "{}{}",
                first.unwrap(),
                digits.last().unwrap_or(first.unwrap())
            )
            .parse::<u32>()
            .unwrap()
        })
        .sum();

    println!("{:?}", result);
}
