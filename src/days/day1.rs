// Helper
fn read_input (input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

// Part1
pub fn part1 (input: &str) -> String {
    let nums = read_input(input);
    for n1 in &nums {
        for n2 in &nums {
            if n1 + n2 == 2020 {
                return format!("{}", n1 * n2)
            }
        }
    }
    return String::from("")
}

// Part2
pub fn part2 (input: &str) -> String {
    let nums = read_input(input);
    for n1 in &nums {
        for n2 in &nums {
            for n3 in &nums {
                if n1 + n2 + n3 == 2020 {
                    return format!("{}", n1 * n2 * n3)
                }
            }
        }
    }
    return String::from("")
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day1_part1 () {
        let input = "1721
979
366
299
675
1456";
        assert_eq!(super::part1(input), "514579");
    }

    #[test]
    fn day1_part2 () {
        let input = "1721
979
366
299
675
1456";
        assert_eq!(super::part2(input), "241861950");
    }
}
