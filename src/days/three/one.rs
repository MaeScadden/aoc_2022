const INPUT: &str = include_str!("input");

pub fn main() -> i32 {
    solve(INPUT)
}

fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(Compartments::from_str)
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
}

impl<'a> Compartments<'a> {
    pub fn from_str(string: &'a str) -> Self {
        let length = string.len();

        if length % 2 != 0 {
            unreachable!("Value: `{}` has odd length {}", string, length,)
        }

        let half = length / 2;
        Self {
            first: string.get(..half).expect("unable to grab first half"),
            second: string.get(half..).expect("unable to grab second half"),
        }
    }

    pub fn shared_char(&self) -> char {
        let shared_char = self.first.chars().find(|first| {
            self.second
                .chars()
                .find(|second| second.eq(&first))
                .is_some()
        });

        if let Some(shared) = shared_char {
            return shared;
        }

        unreachable!(
            "Unable to find shared char for first[{}] and second[{}]",
            self.first, self.second
        )
    }
}

#[cfg(test)]
mod tests {
    mod compartments {
        mod from_str {

            use crate::days::three::one::Compartments;
            #[test]
            fn should_place_first_half_as_attribute_first_value() {
                let first = "aaa";
                let compartments = Compartments::from_str("aaabbb");

                assert_eq!(first, compartments.first);
            }

            #[test]
            fn should_place_second_half_as_attribute_second_value() {
                let second = "bbb";
                let compartments = Compartments::from_str("aaabbb");

                assert_eq!(second, compartments.second);
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

            assert_eq!(solve(input.as_str()), 157)
        }
    }

    mod alphabetical_position {
        use crate::days::three::one::alphabetical_position;

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
