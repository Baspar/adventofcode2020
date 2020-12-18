use itertools::Itertools;

// Helper
fn read_input (input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|n| n.parse().unwrap())
        .collect()
}
fn find_invalid_number (input: &str) -> Option<u64> {
    let data = read_input(input);
    for i in 25..data.len() {
        if let None = data[i-25..i]
            .iter()
            .permutations(2)
            .map(|x| x.iter().cloned().sum::<u64>())
            .find(|x| data[i] == *x) {
                return Some(data[i])
        }
    }
    return None
}

// Part1
pub fn part1 (input: &str) -> String {
    format!("{}", find_invalid_number(input).unwrap_or(0))
}

// Part2
pub fn part2 (input: &str) -> String {
    let data = read_input(input);
    let invalid_number = find_invalid_number(input).unwrap();
    println!("HEY1");
    for start in 0..data.len() {
        let mut tot = data[start];
        for end in start+1..data.len() {
            if tot == invalid_number {
                let min = data[start..end].iter().min().unwrap();
                let max = data[start..end].iter().max().unwrap();
                return format!("{}", min + max)
            }
            if tot > invalid_number {
                break
            }
            tot += data[end];
        }
    }
    format!("{}", 0)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day9_part1 () {
        assert_eq!(super::part1("0"), "0");
    }

    #[test]
    fn day9_part2 () {
        assert_eq!(super::part2("0"), "0");
    }
}
