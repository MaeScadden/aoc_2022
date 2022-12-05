const INPUT: &str = include_str!("input");

pub fn main() -> i32 {
    solve(INPUT, 3)
}

fn solve(input: &str, size: usize) -> i32 {
    let mut biggest_calories = vec![0; size];

    let mut current_calorie = 0;
    let mut lines = input.lines().peekable();

    loop {
        match lines.next() {
            None => break,
            Some(line) => {
                if line.len() != 0 {
                    current_calorie += line.parse::<i32>().unwrap();

                    if lines.peek().is_some() {
                        continue;
                    }
                }

                let smallest_calorie_index = {
                    let mut index = None;
                    let mut smallest = current_calorie;

                    for (i, a_biggest_calorie) in biggest_calories.iter().enumerate() {
                        let value = *a_biggest_calorie;
                        if value < smallest {
                            smallest = value;
                            index = Some(i);
                        }
                    }

                    index
                };

                if let Some(index) = smallest_calorie_index {
                    biggest_calories[index] = current_calorie
                }

                current_calorie = 0
            }
        }
    }

    biggest_calories.iter().fold(0, |total, num| total + num)
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

            assert_eq!(solve(input.as_str(), 3), 45000)
        }
    }
}
