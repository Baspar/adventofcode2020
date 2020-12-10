// Helper
fn read_input (input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line
            .chars()
            .map(|c| c == '#')
            .collect()
        )
        .collect()
}

// Part1
pub fn part1 (input: &str) -> String {
    let is_tree = read_input(input);
    let mut y = 0;
    let mut tot = 0;
    for row in is_tree {
        if row[y] {
            tot += 1;
        }
        y = (y + 3) % row.len();
    }
    format!("{}", tot)
}

// Part2
pub fn part2 (input: &str) -> String {
    let is_tree = read_input(input);
    let w = is_tree[0].len();
    let deltas = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];
    let trees = deltas
        .iter()
        .map(|(dx, dy)|{
            let mut x = 0;
            let mut y = 0;
            let mut tot = 0;
            while y < is_tree.len() {
                if is_tree[y][x] {
                    tot += 1;
                }
                x += dx;
                x %= w;
                y += dy;
            }
            return tot;
        })
        .fold(1u64, |acc, x| acc * x);
    format!("{}", trees)
}

// Tests
#[cfg(test)]
mod tests {
    const INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn day3_part1 () {
        assert_eq!(super::part1(INPUT), "7");
    }

    #[test]
    fn day3_part2 () {
        assert_eq!(super::part2(INPUT), "336");
    }
}
