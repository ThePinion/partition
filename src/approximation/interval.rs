use super::MultiplicativeBoundedMerger;

pub enum SumsetEpsilonAdditiveAproximation {}

impl SumsetEpsilonAdditiveAproximation {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(epsilon_inv: u64) -> SumsetIntervalApproximation {
        SumsetIntervalApproximation::new(epsilon_inv, 1.0 / (epsilon_inv as f64))
    }
}

pub struct SumsetIntervalApproximation {
    start: u64,
    delta: f64,
}

impl SumsetIntervalApproximation {
    pub fn new(start: u64, delta: f64) -> Self {
        Self { start, delta }
    }
    pub fn approximate(&self, set: &[u64]) -> Vec<u64> {
        let n = set.len();
        let delta = self.delta / (n as f64).log2().ceil();
        for x in set {
            assert!(
                self.start <= *x && *x <= self.start * 2,
                "{}, {}",
                self.start,
                x
            )
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

    fn verify_interval_approximation(set: Vec<u64>, delta: f64) {
        let start = set.iter().min().copied().unwrap_or(0);
        let end = set.iter().max().copied().unwrap_or(0);
        assert!(start * 2 >= end);
        let approximation = SumsetIntervalApproximation::new(start, delta).approximate(&set);
        helpers::test::verify_approximation(&approximation, &naive_sumset(&set), delta, 0);
    }

    fn verify_epsilon_additive_approximation(set: Vec<u64>, epsilon_inv: u64) {
        for &i in &set {
            assert!(epsilon_inv <= i && i < epsilon_inv * 2);
        }
        let approximation = SumsetEpsilonAdditiveAproximation::new(epsilon_inv).approximate(&set);
        helpers::test::verify_approximation(
            &approximation,
            &naive_sumset(&set),
            0.0,
            set.len() as u64,
        );
    }

    #[test]
    fn test_interval_approximation() {
        verify_interval_approximation(vec![5, 6, 7, 8, 9, 10], 0.1);
        verify_interval_approximation(vec![5, 6, 7, 8, 9, 10], 0.01);
        verify_interval_approximation(vec![10, 12, 13, 14, 15, 16, 17, 18, 19, 11], 0.001);
        verify_interval_approximation(vec![10, 12, 13, 14, 15, 16, 17, 18, 19, 11], 0.0001);
        verify_interval_approximation(vec![10, 12, 13, 14, 15, 16, 17, 18, 19, 11], 0.5);
    }
    #[test]
    fn test_interval_approximation_large() {
        verify_interval_approximation(
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
    #[test]
    fn test_interval_epsilon_approximation() {
        verify_epsilon_additive_approximation(vec![6, 7, 8, 9, 10, 11], 6);
        verify_epsilon_additive_approximation((12..24).collect(), 12);
    }
}
