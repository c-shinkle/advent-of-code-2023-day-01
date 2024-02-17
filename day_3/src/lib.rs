pub mod input;

use ndarray::{Array, Array2, ArrayView1, Axis};
use regex::Regex;

pub fn get_all_part_numbers_func(mut input: &str) -> u32 {
    let regex = Regex::new(r"(\d+)").unwrap();
    input = input.trim();
    let row_len = input.matches('\n').count() + 1;
    let col_len = input.find('\n').unwrap_or(input.len());
    let vec: Vec<u8> = input.lines().flat_map(|line| line.bytes()).collect();
    let matrix: Array2<u8> = Array2::from_shape_vec((row_len, col_len), vec).unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| regex.find_iter(line).map(move |capture| (row, capture)))
        .map(|(row, capture)| {
            let col = capture.start();
            let len = capture.len();
            let symbol = |byte: &u8| !byte.is_ascii_digit() && *byte != b'.';
            let skip = row.saturating_sub(1);
            let take = if row == 0 { 2 } else { 3 };

            let row_above = find_byte_matrix_row(row.checked_sub(1), &matrix)
                .map(|row| row.iter().skip(col).take(len).any(symbol))
                .unwrap_or(false);
            let row_below = find_byte_matrix_row(Some(row + 1), &matrix)
                .map(|row| row.iter().skip(col).take(len).any(symbol))
                .unwrap_or(false);
            let col_left = find_byte_matrix_col(col.checked_sub(1), &matrix)
                .map(|col| col.iter().skip(skip).take(take).any(symbol))
                .unwrap_or(false);
            let col_right = find_byte_matrix_col(Some(col + len), &matrix)
                .map(|col| col.iter().skip(skip).take(take).any(symbol))
                .unwrap_or(false);

            if !(row_above || row_below || col_left || col_right) {
                return 0;
            }
            capture.as_str().parse::<u32>().unwrap()
        })
        .sum()
}

pub fn get_all_part_numbers_impr(mut input: &str) -> u32 {
    let mut sum = 0;
    let regex = Regex::new(r"(\d+)").unwrap();
    input = input.trim();
    let row_len = input.matches('\n').count() + 1;
    let col_len = input.find('\n').unwrap_or(input.len());
    let mut matrix: Array2<u8> = Array::zeros((row_len, col_len));
    for (i, line) in input.lines().enumerate() {
        matrix.row_mut(i).assign(&Array::from_iter(line.bytes()));
    }
    for (row, line) in input.lines().enumerate() {
        for capture in regex.find_iter(line) {
            let col = capture.start();
            let len = capture.len();
            let symbol = |byte: &u8| !byte.is_ascii_digit() && *byte != b'.';
            let skip = row.saturating_sub(1);
            let take = if row == 0 { 2 } else { 3 };

            let row_above = find_byte_matrix_row(row.checked_sub(1), &matrix)
                .map(|row| row.iter().skip(col).take(len).any(symbol))
                .unwrap_or(false);
            let row_below = find_byte_matrix_row(Some(row + 1), &matrix)
                .map(|row| row.iter().skip(col).take(len).any(symbol))
                .unwrap_or(false);
            let col_left = find_byte_matrix_col(col.checked_sub(1), &matrix)
                .map(|col| col.iter().skip(skip).take(take).any(symbol))
                .unwrap_or(false);
            let col_right = find_byte_matrix_col(Some(col + len), &matrix)
                .map(|col| col.iter().skip(skip).take(take).any(symbol))
                .unwrap_or(false);

            if row_above || row_below || col_left || col_right {
                sum += capture.as_str().parse::<u32>().unwrap();
            }
        }
    }
    sum
}

pub fn no_regex(mut input: &str) -> u32 {
    let mut sum = 0;
    input = input.trim();
    let row_len = input.matches('\n').count() + 1;
    let col_len = input.find('\n').unwrap_or(input.len());
    let mut matrix: Array2<u8> = Array::zeros((row_len, col_len));
    for (i, line) in input.lines().enumerate() {
        matrix.row_mut(i).assign(&Array::from_iter(line.bytes()));
    }
    for (row, line) in input.lines().enumerate() {
        let mut previous_byte_offset = 0;
        for part_str in line
            .split(|c: char| c == '.' || !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
        {
            let len = part_str.len();
            let substring = &line[previous_byte_offset..line.len()];
            let col = substring.find(part_str).unwrap() + previous_byte_offset;
            previous_byte_offset = col + len;

            let symbol = |byte: &u8| !byte.is_ascii_digit() && *byte != b'.';
            let skip = row.saturating_sub(1);
            let take = if row == 0 { 2 } else { 3 };

            let row_above = find_byte_matrix_row(row.checked_sub(1), &matrix)
                .map(|row| row.iter().skip(col).take(len).any(symbol))
                .unwrap_or(false);
            let row_below = find_byte_matrix_row(Some(row + 1), &matrix)
                .map(|row| row.iter().skip(col).take(len).any(symbol))
                .unwrap_or(false);
            let col_left = find_byte_matrix_col(col.checked_sub(1), &matrix)
                .map(|col| col.iter().skip(skip).take(take).any(symbol))
                .unwrap_or(false);
            let col_right = find_byte_matrix_col(Some(col + len), &matrix)
                .map(|col| col.iter().skip(skip).take(take).any(symbol))
                .unwrap_or(false);

            if row_above || row_below || col_left || col_right {
                sum += part_str.parse::<u32>().unwrap();
            }
        }
    }
    sum
}

pub fn no_matrix(mut input: &str) -> u32 {
    let mut sum = 0;
    let regex = Regex::new(r"(\d+)").unwrap();
    input = input.trim();
    let matrix: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    for (row_index, line) in input.lines().enumerate() {
        for capture in regex.find_iter(line) {
            let col_index = capture.start();
            let len = capture.len();
            let skip = row_index.saturating_sub(1);
            let take = if row_index == 0 { 2 } else { 3 };

            let row_above =
                row_symbol(&matrix, row_index.checked_sub(1), col_index, take).unwrap_or(false);
            let row_below =
                row_symbol(&matrix, row_index.checked_add(1), col_index, take).unwrap_or(false);
            let col_left =
                col_symbol(&matrix, col_index.checked_sub(1), skip, take).unwrap_or(false);
            let col_right =
                col_symbol(&matrix, col_index.checked_add(len), skip, take).unwrap_or(false);

            if row_above || row_below || col_left || col_right {
                sum += capture.as_str().parse::<u32>().unwrap();
            }
        }
    }
    sum
}

pub fn no_matrix_or_regex(mut input: &str) -> u32 {
    let mut sum = 0;
    input = input.trim();
    let matrix: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    for (row_index, line) in input.lines().enumerate() {
        let mut previous_byte_offset = 0;
        for part_str in line
            .split(|c: char| c == '.' || !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
        {
            let len = part_str.len();
            let substring = &line[previous_byte_offset..line.len()];
            let col_index = substring.find(part_str).unwrap() + previous_byte_offset;
            previous_byte_offset = col_index + len;
            let skip = row_index.saturating_sub(1);
            let take = if row_index == 0 { 2 } else { 3 };

            let row_above =
                row_symbol(&matrix, row_index.checked_sub(1), col_index, take).unwrap_or(false);
            let row_below =
                row_symbol(&matrix, row_index.checked_add(1), col_index, take).unwrap_or(false);
            let col_left =
                col_symbol(&matrix, col_index.checked_sub(1), skip, take).unwrap_or(false);
            let col_right =
                col_symbol(&matrix, col_index.checked_add(len), skip, take).unwrap_or(false);

            if row_above || row_below || col_left || col_right {
                sum += part_str.parse::<u32>().unwrap();
            }
        }
    }
    sum
}

#[inline(always)]
fn find_byte_matrix_row(i: Option<usize>, matrix: &Array2<u8>) -> Option<ArrayView1<u8>> {
    if i? >= matrix.len_of(Axis(0)) {
        return None;
    }
    Some(matrix.row(i?))
}

#[inline(always)]
fn find_byte_matrix_col(i: Option<usize>, matrix: &Array2<u8>) -> Option<ArrayView1<u8>> {
    if i? >= matrix.len_of(Axis(1)) {
        return None;
    }
    Some(matrix.column(i?))
}

#[inline(always)]
fn row_symbol(
    matrix: &[Vec<u8>],
    row_index: Option<usize>,
    skip: usize,
    take: usize,
) -> Option<bool> {
    let any = matrix
        .get(row_index?)?
        .iter()
        .skip(skip)
        .take(take)
        .any(|byte: &u8| !byte.is_ascii_digit() && *byte != b'.');
    Some(any)
}

#[inline(always)]
fn col_symbol(
    matrix: &[Vec<u8>],
    col_index: Option<usize>,
    skip: usize,
    take: usize,
) -> Option<bool> {
    let col_index = col_index?;
    let any = matrix
        .iter()
        .skip(skip)
        .take(take)
        .filter_map(|row| row.get(col_index))
        .any(|byte: &u8| !byte.is_ascii_digit() && *byte != b'.');
    Some(any)
}

#[cfg(test)]
mod test {
    use crate::input::INPUT;

    use super::*;
    #[test]
    fn test_all_edge_cases() {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
......&...
.2...2....
";

        let actual = get_all_part_numbers_func(input);

        assert_eq!(actual, 4363);
    }

    #[test]
    fn test_real_input() {
        let input = INPUT;

        let actual: u32 = get_all_part_numbers_func(input);

        assert_eq!(actual, 530495);
    }

    #[test]
    fn test_all_edge_cases_impr() {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
......&...
.2...2....
";

        let actual = get_all_part_numbers_impr(input);

        assert_eq!(actual, 4363);
    }

    #[test]
    fn test_real_input_impr() {
        let input = INPUT;

        let actual: u32 = get_all_part_numbers_impr(input);

        assert_eq!(actual, 530495);
    }

    #[test]
    fn no_regex_all_edge_cases() {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
......&...
.2...2....
";

        let actual = no_regex(input);

        assert_eq!(
            actual,
            vec![467, 35, 633, 617, 592, 755, 664, 598, 2]
                .iter()
                .sum::<u32>()
        );
    }

    #[test]
    fn no_regex_real_input() {
        let input = INPUT;

        let actual = no_regex(input);

        assert_eq!(actual, 530495);
    }

    #[test]
    fn no_matrix_all_edge_cases() {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
......&...
.2...2....
";

        let actual = no_matrix(input);

        assert_eq!(
            actual,
            vec![467, 35, 633, 617, 592, 755, 664, 598, 2]
                .iter()
                .sum::<u32>()
        );
    }

    #[test]
    fn no_matrix_real_input() {
        let input = INPUT;

        let actual = no_matrix(input);

        assert_eq!(actual, 530495);
    }

    #[test]
    fn no_matrix_or_regex_all_edge_cases() {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
......&...
.2...2....
";

        let actual = no_matrix_or_regex(input);

        assert_eq!(
            actual,
            vec![467, 35, 633, 617, 592, 755, 664, 598, 2]
                .iter()
                .sum::<u32>()
        );
    }

    #[test]
    fn no_matrix_or_regex_real_input() {
        let input = INPUT;

        let actual = no_matrix_or_regex(input);

        assert_eq!(actual, 530495);
    }
}
