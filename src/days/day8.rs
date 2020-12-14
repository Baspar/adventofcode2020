use onig::Regex;
use std::collections::HashSet;

#[derive(Debug,Clone)]
enum Instr {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

// Helper
fn read_input (input: &str) -> Vec<Instr> {
    let r = Regex::new("(nop|acc|jmp) ([+-][0-9]*)").unwrap();
    return input
        .lines()
        .map(|line| {
            let caps = r.captures(line).unwrap();
            let groups: Vec<&str> = caps
                .iter()
                .map(|x| x.unwrap())
                .collect();
            let n: i64 = groups[2].parse().unwrap();
            return match groups[1] {
                "acc" => Instr::Acc(n),
                "jmp" => Instr::Jmp(n),
                _     => Instr::Nop(n),
            }
        })
        .collect()
}

// Part1
pub fn part1 (input: &str) -> String {
    let instructions = read_input(input);
    let mut already_executed: HashSet<i64> = HashSet::new();
    let mut i = 0i64;
    let mut acc = 0;
    while !already_executed.contains(&i) {
        already_executed.insert(i.clone());
        match instructions[i as usize] {
            Instr::Nop(_) => { i += 1; },
            Instr::Acc(n) => { acc += n; i += 1; }
            Instr::Jmp(n) => { i += n; }
        }
    }
    format!("{}", acc)
}

// Part2
pub fn part2 (input: &str) -> String {
    let mut instructions = read_input(input);
    for instr_to_swap in 0..instructions.len() {
        let orig = instructions[instr_to_swap].clone();

        match orig {
            Instr::Nop(n) => { instructions[instr_to_swap] = Instr::Jmp(n) },
            Instr::Jmp(n) => { instructions[instr_to_swap] = Instr::Nop(n) },
            Instr::Acc(_) => { continue },
        }

        let mut already_executed: HashSet<i64> = HashSet::new();
        let mut i = 0i64;
        let mut acc = 0;
        while !already_executed.contains(&i) {
            already_executed.insert(i.clone());
            if let Some(instruction) = instructions.get(i as usize) {
                match instruction {
                    Instr::Nop(_) => { i += 1; },
                    Instr::Acc(n) => { acc += n; i += 1; }
                    Instr::Jmp(n) => { i += n; }
                }
            } else {
                return format!("{}", acc)
            }
        }

        instructions[instr_to_swap] = orig;
    }
    return format!("Error")
}

// Tests
#[cfg(test)]
mod tests {
    const INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    #[test]
    fn day8_part1 () {
        assert_eq!(super::part1(INPUT), "5");
    }

    #[test]
    fn day8_part2 () {
        assert_eq!(super::part2(INPUT), "8");
    }
}
