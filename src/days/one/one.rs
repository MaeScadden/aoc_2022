const INPUT: &str = include_str!("input");

pub fn main() -> i32 {
    solve(INPUT)
}

fn solve(input: &str) -> i32 {
    input
        .lines()
        .fold((0, 0), |(biggest, current), line| {
            if line.len() != 0 {
                (
                    biggest,
                    current + line.parse::<i32>().expect("Failed to string to int"),
                )
            } else if biggest < current {
                (current, 0)
            } else {
                (biggest, 0)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    mod given_sample_input {
        use super::super::solve;
        use crate::builder;

        #[test]
        fn should_return_expected_value() {
            let mut builder = builder::Builder::default();

            let input = builder
                .append("1000\n")
                .append("2000\n")
                .append("3000\n\n")
                .append("4000\n\n")
                .append("5000\n")
                .append("6000\n\n")
                .append("7000\n")
                .append("8000\n")
                .append("9000\n\n")
                .append("10000\n")
                .string();

            assert_eq!(solve(input.as_str()), 24000)
        }
    }
}
