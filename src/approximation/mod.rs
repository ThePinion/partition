use multiplicative::MultiplicativeBoundedMerger;

pub mod additive;
pub mod multiplicative;

pub struct SumsetApproximation {
    start: u64,
    delta: f64,
}

impl SumsetApproximation {
    pub fn new(start: u64, delta: f64) -> Self {
        Self { start, delta }
    }
    pub fn approximate(&self, set: &[u64]) -> Vec<u64> {
        let n = set.len();
        let delta = self.delta / (n as f64).log2().ceil();
        for x in set {
            assert!(self.start <= *x && *x <= self.start * 2)
        }
        self.approximate_recursive(set, dbg!(delta))
    }
    fn approximate_recursive(&self, a: &[u64], delta: f64) -> Vec<u64> {
        if a.len() <= 1 {
            return a.to_vec();
        }
        let length = a.len();
        let pivot = length / 2;
        let (left, right) = a.split_at(pivot);
        let left_approximation = self.approximate_recursive(left, delta);
        let right_approximation = self.approximate_recursive(right, delta);
        dbg!(&left_approximation, &right_approximation);
        let merger = MultiplicativeBoundedMerger::new(
            self.start,
            self.start,
            delta,
            length as u64 * self.start * 2,
        );
        [
            merger.merge(&left_approximation, &right_approximation),
            left_approximation,
            right_approximation,
        ]
        .concat()
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::{self, test::naive_sumset};

    use super::*;

    fn verify_approximation(set: Vec<u64>, delta: f64) {
        let start = set.iter().min().copied().unwrap_or(0);
        let end = set.iter().max().copied().unwrap_or(0);
        assert!(start * 2 >= end);
        let approximation = SumsetApproximation::new(start, delta).approximate(&set);
        helpers::test::verify_approximation(&approximation, &naive_sumset(&set), delta, 0);
    }

    #[test]
    fn test_approximate() {
        verify_approximation(vec![5, 6, 7, 8, 9, 10], 0.1);
        verify_approximation(vec![5, 6, 7, 8, 9, 10], 0.01);
        verify_approximation(vec![10, 12, 13, 14, 15, 16, 17, 18, 19, 11], 0.001);
        verify_approximation(vec![10, 12, 13, 14, 15, 16, 17, 18, 19, 11], 0.0001);
        verify_approximation(vec![10, 12, 13, 14, 15, 16, 17, 18, 19, 11], 0.5);
    }
    #[test]
    fn test_approximate_large() {
        verify_approximation(
            vec![
                200, 120, 130, 140, 150, 160, 170, 180, 190, 210, 121, 123, 124, 125, 126, 126,
                126, 126, 126, 126, 126, 126,
            ]
            .into_iter()
            .map(|x| x * 1000000000)
            .collect(),
            0.1,
        );
    }
}
