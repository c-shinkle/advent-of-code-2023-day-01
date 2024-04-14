use super::*;

struct MappingStruct {
    dest: Index,
    src: Index,
    len: Index,
}

impl FromIterator<Index> for MappingStruct {
    fn from_iter<T: IntoIterator<Item = Index>>(iter: T) -> Self {
        let mut into_iter = iter.into_iter();

        MappingStruct {
            dest: into_iter.next().unwrap(),
            src: into_iter.next().unwrap(),
            len: into_iter.next().unwrap(),
        }
    }
}

impl From<MappingStruct> for Mapping {
    fn from(MappingStruct { dest, src, len }: MappingStruct) -> Self {
        (dest, src, len)
    }
}

fn take_mappings_from_lines(lines: &[&str]) -> Vec<Mapping> {
    lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|number_str| number_str.parse().unwrap())
                .collect::<MappingStruct>()
                .into()
        })
        .collect()
}

fn get_mapping(mappings: &[Mapping], value: Index) -> Index {
    mappings
        .iter()
        .find(|&&(_, src, len)| (src..src + len).contains(&value))
        .map(|&(dest, src, _)| value - src + dest)
        .unwrap_or(value)
}

pub fn part_1(input: &str) -> Index {
    let lines: Vec<&str> = input.trim().lines().collect();
    let list_of_mappings: Vec<Vec<Mapping>> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("map:"))
        .map(|(offset, _)| take_mappings_from_lines(&lines[(offset + 1)..]))
        .collect();

    let seed_line = lines[0];
    seed_line[(seed_line.find(':').unwrap() + 1)..]
        .split_ascii_whitespace()
        .map(|seed_str| {
            let seed = seed_str.parse().unwrap();

            let soil = get_mapping(&list_of_mappings[0], seed);
            let fertilizer = get_mapping(&list_of_mappings[1], soil);
            let water = get_mapping(&list_of_mappings[2], fertilizer);
            let light = get_mapping(&list_of_mappings[3], water);
            let temp = get_mapping(&list_of_mappings[4], light);
            let humidity = get_mapping(&list_of_mappings[5], temp);
            get_mapping(&list_of_mappings[6], humidity)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::INPUT;

    #[test]
    fn func_sample() {
        let actual = part_1(EXAMPLE);
        assert_eq!(actual, 35);
    }

    #[test]
    fn func_puzzle_input() {
        let actual = part_1(INPUT);
        assert_eq!(actual, 107430936);
    }
}