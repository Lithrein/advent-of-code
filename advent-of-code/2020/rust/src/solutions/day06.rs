use crate::solver::Solver;
use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader},
    iter::FromIterator,
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<HashSet<char>>>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines()
            .flatten()
            .fold(vec![vec![]], |mut acc, s| {
                if s == "" {
                    acc.push(vec![]);
                } else {
                    let len = acc.len() - 1;
                    acc[len].push(s);
                }
                acc
            })
            .iter()
            .map(|group| {
                group
                    .iter()
                    .map(|line| HashSet::from_iter(line.chars()))
                    .collect()
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().map(|answers| first_part(answers)).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().map(|answers| second_part(answers)).sum()
    }
}

fn first_part(answers: &[HashSet<char>]) -> usize {
    let mut answers_iter = answers.iter();
    let init = answers_iter.next().unwrap_or(&HashSet::new()).clone();
    answers_iter.fold(init, |acc, set| &acc | set).len()
}

fn second_part(answers: &[HashSet<char>]) -> usize {
    let mut answers_iter = answers.iter();
    let init = answers_iter.next().unwrap_or(&HashSet::new()).clone();
    answers_iter.fold(init, |acc, set| &acc & set).len()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day01::*;

    #[test]
    fn test_first_part() {}

    #[test]
    fn test_second_part() {}
}
