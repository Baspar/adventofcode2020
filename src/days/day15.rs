// Helper
fn read_input (_input: &str) -> i32 {
    0
}

// Part1
pub fn part1 (input: &str) -> String {
    format!("{}", read_input(input))
}

// Part2
pub fn part2 (input: &str) -> String {
    format!("{}", read_input(input))
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day15_part1 () {
        assert_eq!(super::part1("0"), "0");
    }

    #[test]
    fn day15_part2 () {
        assert_eq!(super::part2("0"), "0");
    }
}
