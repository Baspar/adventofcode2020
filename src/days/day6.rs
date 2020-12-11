use std::collections::HashSet;

// Helper
// fn read_input (input: &str) -> i32 {
// }

// Part1
pub fn part1 (input: &str) -> String {
    let common_answer = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|line| line.chars())
                .collect()
        })
        .fold(0, |tot, group: HashSet<_>| tot + group.len());
    format!("{}", common_answer)
}

// Part2
pub fn part2 (input: &str) -> String {
    let common_answer: usize = input
        .split("\n\n")
        .map(|group| {
            let all_answers: Vec<HashSet<char>> = group
                .lines()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .collect();
            let mut out = None;
            for ans in all_answers {
                match &out {
                    None => { out = Some(ans) },
                    Some(a) => {
                        let inter = a.intersection(&ans).map(|x| *x).collect();
                        out = Some(inter);
                    }
                }
            }
            return match out {
                None => 0,
                Some(ans) => ans.len()
            }
        })
        .sum();
    format!("{}", common_answer)
}

// Tests
#[cfg(test)]
mod tests {
    const INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";
    #[test]
    fn day6_part1 () {
        assert_eq!(super::part1(INPUT), "11");
    }

    #[test]
    fn day6_part2 () {
        assert_eq!(super::part2(INPUT), "6");
    }
}
