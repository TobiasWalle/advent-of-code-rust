use std::collections::HashSet;
use utils;

fn execute_line(frequency: i32, line: &str) -> i32 {
    let operator: &str = &line[0..1];
    let number: i32 = (&line[1..]).parse().unwrap();
    if operator == "+" {
        frequency + number
    } else if operator == "-" {
        frequency - number
    } else {
        println!("Unknown symbole '{}'", line);
        frequency
    }
}

fn calculate_frequency(input: &str) -> i32 {
    let lines = input.split("\n");
    lines.fold(0, &execute_line)
}

fn get_repeating_frequency(input: &str) -> i32 {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut frequencies = HashSet::new();
    let mut frequency = 0;
    frequencies.insert(frequency);
    let mut index = 0;
    loop {
        frequency = execute_line(frequency, &lines[index]);
        if frequencies.contains(&frequency) {
            return frequency
        }
        frequencies.insert(frequency);

        index += 1;
        if index >= lines.len() {
            index = 0;
        }
    }
}

pub fn solve_1() {
    utils::solve("./inputs/day1", &calculate_frequency);
}

pub fn solve_2() {
    utils::solve("./inputs/day1", &get_repeating_frequency);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_frequency() {
        let input = "\
+1
+4
-2";
        assert_eq!(calculate_frequency(input), 3)
    }

    #[test]
    fn test_get_repeating_frequency_1() {
        let input = "\
+1
-2
+3
+1";
        assert_eq!(get_repeating_frequency(input), 2)
    }

    #[test]
    fn test_get_repeating_frequency_2() {
        let input = "\
+1
-1";
        assert_eq!(get_repeating_frequency(input), 0)
    }

    #[test]
    fn test_get_repeating_frequency_3() {
        let input = "\
+7
-7
-2
-7
-4";
        assert_eq!(get_repeating_frequency(input), 0)
    }
}
