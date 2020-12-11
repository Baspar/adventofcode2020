use std::collections::HashSet;

// Helper
fn read_input (input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line
            .chars()
            .map(|c| c == 'R' || c == 'B')
            .collect()
        )
        .collect()
}

// Part1
pub fn part1 (input: &str) -> String {
    let instructions = read_input(input);
    let max_seat = instructions
        .iter()
        .map(|instr| instr
            .iter()
            .fold(0, |acc, &bit| {
                2 * acc + if bit {1} else {0}
            }))
        .max()
        .unwrap();
    format!("{}", max_seat)
}

// Part2
pub fn part2 (input: &str) -> String {
    let instructions = read_input(input);
    let seats: HashSet<usize> = instructions
        .iter()
        .map(|instr| instr
            .iter()
            .fold(0, |acc, &bit| {
                2 * acc + if bit {1} else {0}
            }))
        .collect();
    for seat in 0..1024 {
        if !seats.contains(&(seat as usize)) {
            println!("{}", seat);
        }
    }
    format!("{}", 0)
}

// Tests
#[cfg(test)]
mod tests {
    const INPUT: &str = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn day5_part1 () {
        assert_eq!(super::part1(INPUT), "820");
    }

    #[test]
    fn day5_part2 () {
        assert_eq!(super::part2("0"), "0");
    }
}
