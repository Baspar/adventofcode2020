use std::collections::{HashMap, HashSet};
use onig::Regex;


type Containing = HashMap<String, HashSet<String>>;
type Contains = HashMap<String, HashMap<String, usize>>;

// Helper
fn read_input (input: &str) -> (Containing, Contains) {
    let r: Regex = Regex::new(r#"([1-9][0-9]*) ([^,\.]*) bags?"#).unwrap();
    let mut containing: Containing = HashMap::new();
    let contains: Contains = input
        .lines()
        .map(|line| {
            let data: Vec<&str> = line.split(" bags contain ").collect();
            let bag = data[0].to_string();
            let mut contains: HashMap<String, usize> = HashMap::new();
            if data[1] != "no other bags." {
                data[1]
                    .split(", ")
                    .for_each(|s| {
                        let parts: Vec<String> = r.captures(s)
                            .unwrap()
                            .iter()
                            .map(|c| c.unwrap().to_string())
                            .collect();
                        let quantity: usize = parts[1].parse().unwrap();
                        let color = parts[2].clone();
                        containing
                            .entry(color.clone())
                            .or_insert(HashSet::new())
                            .insert(bag.clone());
                        contains.insert(color, quantity);
                    })
            }
            return (bag, contains)
        })
        .collect();
    (containing, contains)
}

// Part1
pub fn part1 (input: &str) -> String {
    let (containing, _contains) = read_input(input);
    let mut seen: HashSet<String> = HashSet::new();

    let mut q = Vec::new();
    q.push("shiny gold".to_string());

    while let Some(x) = q.pop() {
        if !seen.contains(&x) {
            seen.insert(x.clone());
            if let Some(parents) = containing.get(&x) {
                for parent in parents {
                    q.push(parent.clone());
                }
            }
        }
    }

    format!("{}", seen.len() - 1)
}

// Part2
pub fn part2 (input: &str) -> String {
    let (_containing, contains) = read_input(input);

    let mut tot = 0;
    let mut q = Vec::new();
    q.push((1, "shiny gold".to_string()));

    while let Some((count, color)) = q.pop() {
        tot += count;
        for (child, count_child) in &contains[&color] {
            q.push((count * count_child, child.clone()));
        }
    }

    format!("{}", tot - 1)
}

// Tests
#[cfg(test)]
mod tests {
    const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    #[test]
    fn day7_part1 () {
        assert_eq!(super::part1(INPUT), "4");
    }

    #[test]
    fn day7_part2 () {
        assert_eq!(super::part2(INPUT), "32");
    }
}
