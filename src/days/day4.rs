use onig::Regex;

#[derive(Debug)]
struct Passport {
    pub byr: Option<String>, // Birth Year
    pub iyr: Option<String>, // Issue Year
    pub eyr: Option<String>, // Expiration Year
    pub hgt: Option<String>, // Height
    pub hcl: Option<String>, // Hair Color
    pub ecl: Option<String>, // Eye Color
    pub pid: Option<String>, // Passport ID
    pub cid: Option<String>  // Country ID
}
impl Passport {
    pub fn is_kinda_valid(self: &Self) -> bool {
        return
            self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some()
    }
    fn is_byr_valid(self: &Self) -> bool {
        match &self.byr {
            None => false,
            Some(byr) => match byr.parse::<usize>() {
                Err(_) => false,
                Ok(byr) => byr >= 1920 && byr <= 2002
            }
        }
    }
    fn is_iyr_valid(self: &Self) -> bool {
        match &self.iyr {
            None => false,
            Some(iyr) => match iyr.parse::<usize>() {
                Err(_) => false,
                Ok(iyr) => iyr >= 2010 && iyr <= 2020
            }
        }
    }
    fn is_eyr_valid(self: &Self) -> bool {
        match &self.eyr {
            None => false,
            Some(eyr) => match eyr.parse::<usize>() {
                Err(_) => false,
                Ok(eyr) => eyr >= 2020 && eyr <= 2030
            }
        }
    }
    fn is_hgt_valid(self: &Self) -> bool {
        match &self.hgt {
            None => false,
            Some(hgt) => {
                let r = Regex::new("([0-9]+)(cm|in)").unwrap();
                match r.captures(hgt) {
                    None => false,
                    Some(cap) => {
                        let groups: Vec<&str> = cap.iter().map(|g| g.unwrap()).collect();
                        let n: usize = groups[1].parse().unwrap();
                        let unit = groups[2];
                        match unit {
                            "cm" => n >= 150 && n <= 193,
                            "in" => n >= 59 && n <= 76,
                            _ => false
                        }
                    }
                }
            }
        }
    }
    fn is_hcl_valid(self: &Self) -> bool {
        match &self.hcl {
            None => false,
            Some(hcl) => {
                let r = Regex::new("#[a-z0-9]{6}").unwrap();
                r.is_match(hcl)
            }
        }
    }
    fn is_ecl_valid(self: &Self) -> bool {
        match &self.ecl {
            None => false,
            Some(ecl) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .any(|c| c == ecl)
        }
    }
    fn is_pid_valid(self: &Self) -> bool {
        match &self.pid {
            None => false,
            Some(pid) => {
                let r = Regex::new("[0-9]{9}").unwrap();
                r.is_match(pid)
            }
        }
    }
    fn is_cid_valid(self: &Self) -> bool {
        true
    }
    pub fn is_valid(self: &Self) -> bool {
        self.is_byr_valid() &&
            self.is_iyr_valid() &&
            self.is_eyr_valid() &&
            self.is_hgt_valid() &&
            self.is_hcl_valid() &&
            self.is_ecl_valid() &&
            self.is_pid_valid() &&
            self.is_cid_valid()
    }
    pub fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None
        }
    }
}

// Helper
fn read_input (input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|passport| {
            let mut p = Passport::new();
            passport
                .split(|c| c == ' ' || c == '\n')
                .filter(|s| !s.is_empty())
                .for_each(|entry| {
                    let x: Vec<&str> = entry.split(':').collect();
                    let key = String::from(x[0]);
                    let val = x[1];
                    if key == "byr" { p.byr = Some(String::from(val)) }
                    if key == "iyr" { p.iyr = Some(String::from(val)) }
                    if key == "eyr" { p.eyr = Some(String::from(val)) }
                    if key == "hgt" { p.hgt = Some(String::from(val)) }
                    if key == "hcl" { p.hcl = Some(String::from(val)) }
                    if key == "ecl" { p.ecl = Some(String::from(val)) }
                    if key == "pid" { p.pid = Some(String::from(val)) }
                    if key == "cid" { p.cid = Some(String::from(val)) }
                });
            return p;
        })
        .collect()
}

// Part1
pub fn part1 (input: &str) -> String {
    let nb_passports = read_input(input)
        .iter()
        .fold(0, |nb_valid, pass| {
            if pass.is_kinda_valid() {
                nb_valid + 1
            } else {
                nb_valid
            }
        });
    format!("{}", nb_passports)
}

// Part2
pub fn part2 (input: &str) -> String {
    let nb_passports = read_input(input)
        .iter()
        .fold(0, |nb_valid, pass| {
            if pass.is_valid() {
                nb_valid + 1
            } else {
                nb_valid
            }
        });
    format!("{}", nb_passports)
}

// Tests
#[cfg(test)]
mod tests {

    #[test]
    fn day4_part1 () {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(super::part1(input), "2");
    }

    #[test]
    fn day4_part2 () {
        let all_invalid = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let all_valid = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(super::part2(all_invalid), "0");
        assert_eq!(super::part2(all_valid), "4");
    }
}
