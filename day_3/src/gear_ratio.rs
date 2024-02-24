use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::Range;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Location {
    range: Range<usize>,
    row: usize,
}

pub fn get_all_gear_ratios_func(mut input: &str) -> u32 {
    input = input.trim();
    let mut all_locations: Vec<Location> = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let mut previous_byte_offset = 0;
        for part_str in line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
        {
            let substring = &line[previous_byte_offset..line.len()];
            let start = substring.find(part_str).unwrap() + previous_byte_offset;
            previous_byte_offset = start + part_str.len();
            all_locations.push(Location {
                range: start..start + part_str.len(),
                row,
            });
        }
    }

    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for (asterisk_row, asterisk_col) in lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| line.match_indices('*').map(move |(index, _)| (row, index)))
    {
        let set: HashSet<Location> = [
            (Some(asterisk_row), asterisk_col.checked_add(1)),
            (asterisk_row.checked_sub(1), asterisk_col.checked_add(1)),
            (asterisk_row.checked_sub(1), Some(asterisk_col)),
            (asterisk_row.checked_sub(1), asterisk_col.checked_sub(1)),
            (Some(asterisk_row), asterisk_col.checked_sub(1)),
            (asterisk_row.checked_add(1), asterisk_col.checked_sub(1)),
            (asterisk_row.checked_add(1), Some(asterisk_col)),
            (asterisk_row.checked_add(1), asterisk_col.checked_add(1)),
        ]
        .into_iter()
        .filter_map(|(row, col)| Some((row?, col?)))
        .flat_map(|(row, col)| {
            all_locations.iter().map(move |location| {
                let Location {
                    range,
                    row: digit_row,
                } = location;
                (row == *digit_row && range.contains(&col)).then_some(location.clone())
            })
        })
        .flatten()
        .collect();

        match set.len().cmp(&2) {
            Ordering::Less => {}
            Ordering::Equal => {
                let mut set_iter = set.into_iter();

                let Location { row, range } = set_iter.next().unwrap();
                let first_number_str = &lines[row][range];
                let first_number = first_number_str.parse::<u32>().unwrap();

                let Location { row, range } = set_iter.next().unwrap();
                let second_number_str = &lines[row][range];
                let second_number = second_number_str.parse::<u32>().unwrap();

                sum += first_number * second_number;
            }
            Ordering::Greater => unreachable!(),
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use crate::input::INPUT;

    use super::*;
    #[test]
    fn gear_ratio_func_all_edge_cases() {
        let input = "
467..114..
...*......
..35..633.
......#...
*617......
.....+.58*
..592.....
......755.
...$.*....
.664.598..
";

        let actual = get_all_gear_ratios_func(input);

        assert_eq!(actual, 467835);
    }

    #[test]
    fn gear_ratio_func_real_input() {
        let actual: u32 = get_all_gear_ratios_func(INPUT);

        assert_eq!(actual, 80253814);
    }
}