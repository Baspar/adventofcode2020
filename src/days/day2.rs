use onig::Regex;

type Password = (usize, usize, char, String);

// Helper
fn read_input (input: &str) -> Vec<Password> {
    let r = Regex::new("([0-9]+)-([0-9]+) ([a-z]): (.*)").unwrap();
    return input
        .lines()
        .map(|line| {
            let captures =  r
                .captures(line)
                .unwrap();
            let groups: Vec<&str> = captures
                .iter()
                .map(|x| x.unwrap())
                .collect();
            return (
                groups[1].parse().unwrap(),
                groups[2].parse().unwrap(),
                groups[3].chars().next().unwrap(),
                groups[4].to_string()
            )
        })
        .collect()
}

// Part1
pub fn part1 (input: &str) -> String {
  let passwords = read_input(input);

  let mut correct = 0;
  for (min, max, letter, password) in passwords {
    let count_in_pass = password
      .chars()
      .filter(|c| *c == letter)
      .count();

    if count_in_pass >= min && count_in_pass <= max {
      correct += 1;
    }
  }
  format!("{:?}", correct)
}

// Part2
pub fn part2 (input: &str) -> String {
  let passwords = read_input(input);

  let mut correct = 0;
  for (pos_a, pos_b, letter, password) in passwords {
    let all_chars: Vec<char> = password.chars().collect();
    if (all_chars[pos_a - 1] == letter) ^ (all_chars[pos_b - 1] == letter) {
      correct += 1
    }
  }
  format!("{:?}", correct)
}

// Tests
#[cfg(test)]
mod tests {
    const INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
    #[test]
    fn day2_part1 () {
        assert_eq!(super::part1(INPUT), "2");
    }

    #[test]
    fn day2_part2 () {
        assert_eq!(super::part2(INPUT), "1");
    }
}
