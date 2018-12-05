use std::collections::HashMap;
use std::collections::HashSet;

use utils;
use utils::char_at;

pub fn calculate_checksum(input: &str) -> i32 {
    let lines = input.split('\n');
    let mut count_2 = 0;
    let mut count_3 = 0;
    for line in lines {
        let mut map = HashMap::new();
        for c in line.chars() {
            let count = match map.get(&c) {
                Some(value) => value + 1,
                None => 1
            };
            map.insert(c, count);
        }

        let counts: HashSet<&i32> = map.values().collect();
        if counts.contains(&2) {
            count_2 += 1;
        }
        if counts.contains(&3) {
            count_3 += 1;
        }
    }
    count_2 * count_3
}

pub fn find_common_letters(input: &str) -> String {
    let lines: Vec<&str> = input.split('\n').collect();
    for i in 0..lines.len() {
        'outer: for j in 0..lines.len() {
            if i == j {
                continue;
            }
            let mut result = String::new();
            let mut has_mismatch = false;
            for k in 0..lines[i].len() {
                let c1 = char_at(lines[i], k);
                let c2 = char_at(lines[j], k);
                let are_different = c1 != c2;
                match (are_different, has_mismatch) {
                    (false, _) => result.push(c1),
                    (true, false) => has_mismatch = true,
                    (true, true) => continue 'outer
                }
            }
            if has_mismatch {
                return result;
            }
        }
    }
    "".to_owned()
}

pub fn solve_1() {
    utils::solve("./inputs/day2", &calculate_checksum);
}

pub fn solve_2() {
    utils::solve("./inputs/day2", &find_common_letters);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_checksum() {
        let input = "\
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";
        assert_eq!(calculate_checksum(input), 12)
    }

    #[test]
    fn test_find_common_letters() {
        let input = "\
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";
        assert_eq!(find_common_letters(input), "fgij")
    }
}
