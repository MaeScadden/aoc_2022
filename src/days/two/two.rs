const INPUT: &str = include_str!("input");

pub fn main() -> i32 {
    solve(INPUT)
}

fn solve(input: &str) -> i32 {
    let (player_1_points, player_2_points) = input
        .lines()
        .map(|line| line.split(' '))
        .map(|mut split| {
            (
                split.nth(0).expect("Missing first players move"),
                split.nth(0).expect("Missing second players move"),
            )
        })
        .map(|(player1, player2)| (Hand::from_str(player1), Outcome::from_str(player2)))
        .fold(
            (0, 0),
            |(mut player_1_points, mut player_2_points), (player1, outcome)| {
                let player2 = player1.from_outcome(outcome);

                player_1_points += player1.points_from_playing_hand();
                player_2_points += player2.points_from_playing_hand();

                let (player_1_outcome, player_2_outcome) = Hand::get_outcome(&player1, &player2);

                player_1_points += player_1_outcome;
                player_2_points += player_2_outcome;

                (player_1_points, player_2_points)
            },
        );

    player_1_points.min(player_2_points)
}

#[derive(PartialEq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    pub fn from_str(string: &str) -> Self {
        match string {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            value => unreachable!("Unrecognized outcome: `{}`", value),
        }
    }
}

impl Hand {
    fn wins_against(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn loses_against(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn from_outcome(&self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => self.loses_against(),
            Outcome::Lose => self.wins_against(),
            Outcome::Draw => self.clone(),
        }
    }

    pub fn from_str(string: &str) -> Self {
        match string {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            value => unreachable!("Unrecognized hand: `{}`", value),
        }
    }

    pub fn points_from_playing_hand(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    pub fn get_outcome(a: &Self, b: &Self) -> (i32, i32) {
        if a == b {
            (3, 3)
        } else if a.beats(b) {
            (6, 0)
        } else {
            (0, 6)
        }
    }

    fn beats(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Rock, Self::Scissors) => true,
            (Self::Rock, _) => false,
            (Self::Paper, Self::Rock) => true,
            (Self::Paper, _) => false,
            (Self::Scissors, Self::Paper) => true,
            (Self::Scissors, _) => false,
        }
    }
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
                .append("A Y\n")
                .append("B X\n")
                .append("C Z\n")
                .string();

            assert_eq!(solve(input.as_str()), 12)
        }
    }
}
