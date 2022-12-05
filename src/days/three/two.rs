const INPUT: &str = include_str!("input");

pub fn main() -> i32 {
    solve(INPUT)
}

fn solve(input: &str) -> i32 {
    input
        .lines()
        .fold(vec![vec![]], |mut acc, line| {
            let last = acc.last_mut().unwrap();

            if last.len() != 3 {
                last.push(line)
            } else {
                acc.push(vec![line])
            }

            acc
        })
        .iter()
        .map(Compartments::from_group)
        .map(|compartments| compartments.shared_char())
        .map(|c| {
            let mut value = alphabetical_position(&c);
            if c.is_ascii_uppercase() {
                value += 26
            }

            value
        })
        .sum()
}

fn alphabetical_position(c: &char) -> i32 {
    match c.to_ascii_lowercase() {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        _ => unreachable!("character[{}] is not an alphabetical charaacter", c),
    }
}

struct Compartments<'a> {
    first: &'a str,
    second: &'a str,
    third: &'a str,
}

impl<'a> Compartments<'a> {
    pub fn from_group(group_of_three: &Vec<&'a str>) -> Self {
        if group_of_three.len() != 3 {
            unreachable!("Value: `{:?}` is not length of 3", group_of_three)
        }

        Self {
            first: group_of_three.get(0).unwrap(),
            second: group_of_three.get(1).unwrap(),
            third: group_of_three.get(2).unwrap(),
        }
    }

    pub fn shared_char(&self) -> char {
        // this is O(n^3) (is there a faster way)?
        let shared_char = self.first.chars().find(|first| {
            self.second
                .chars()
                .find(|second| second.eq(&first))
                .and_then(|second| self.third.chars().find(|third| third.eq(&second)))
                .is_some()
        });

        if let Some(shared) = shared_char {
            return shared;
        }

        unreachable!(
            "Unable to find shared char for first[{}] and second[{}] and third[{}]",
            self.first, self.second, self.third
        )
    }
}

#[cfg(test)]
mod tests {
    mod compartments {
        mod from_str {

            use super::super::super::Compartments;
            #[test]
            fn should_place_group_by_index_position() {
                let group = vec!["aaa", "bbb", "ccc"];
                let compartments = Compartments::from_group(&group);

                assert_eq!(*group.get(0).unwrap(), compartments.first);
                assert_eq!(*group.get(1).unwrap(), compartments.second);
                assert_eq!(*group.get(2).unwrap(), compartments.third);
            }
        }
    }

    mod given_sample_input {
        use super::super::solve;
        use crate::builder;

        #[test]
        fn should_return_expected_value() {
            let mut builder = builder::Builder::default();

            let input = builder
                .append("vJrwpWtwJgWrhcsFMMfFFhFp\n")
                .append("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n")
                .append("PmmdzqPrVvPwwTWBwg\n")
                .append("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n")
                .append("ttgJtRGJQctTZtZT\n")
                .append("CrZsJsPPZsGzwwsLwLmpwMDw\n")
                .string();

            assert_eq!(solve(input.as_str()), 70)
        }
    }

    mod alphabetical_position {
        use super::super::alphabetical_position;

        const INPUTS: [(char, i32); 26] = [
            ('a', 1),
            ('b', 2),
            ('c', 3),
            ('d', 4),
            ('e', 5),
            ('f', 6),
            ('g', 7),
            ('h', 8),
            ('i', 9),
            ('j', 10),
            ('k', 11),
            ('l', 12),
            ('m', 13),
            ('n', 14),
            ('o', 15),
            ('p', 16),
            ('q', 17),
            ('r', 18),
            ('s', 19),
            ('t', 20),
            ('u', 21),
            ('v', 22),
            ('w', 23),
            ('x', 24),
            ('y', 25),
            ('z', 26),
        ];

        #[test]
        fn lower_case_has_expected_result() {
            for (c, expected) in INPUTS.iter() {
                assert_eq!(alphabetical_position(&c), *expected)
            }
        }

        #[test]
        fn upper_case_has_expected_result() {
            for (c, expected) in INPUTS.iter() {
                assert_eq!(alphabetical_position(&c.to_ascii_uppercase()), *expected)
            }
        }
    }
}
