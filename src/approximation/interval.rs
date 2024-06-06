use std::marker::PhantomData;

use crate::fft::Convoluter;

use super::MultiplicativeBoundedMerger;

pub enum SumsetEpsilonAdditiveAproximation {}

impl SumsetEpsilonAdditiveAproximation {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: Convoluter>(epsilon_inv: u64) -> SumsetIntervalApproximation<T> {
        SumsetIntervalApproximation::new(epsilon_inv, 1.0 / (epsilon_inv as f64))
    }
}

pub struct SumsetIntervalApproximation<T: Convoluter> {
    start: u64,
    delta: f64,
    _phantom: PhantomData<T>,
}

impl<T: Convoluter> SumsetIntervalApproximation<T> {
    pub fn new(start: u64, delta: f64) -> Self {
        Self {
            start,
            delta,
            _phantom: PhantomData,
        }
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
        self.approximate_recursive(set, delta)
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
        let merger = MultiplicativeBoundedMerger::<T>::new(
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
    use crate::{
        fft::{FFT, NTT},
        helpers::{self, test::naive_sumset},
    };

    use super::*;

    fn verify_interval_approximation<T: Convoluter>(set: Vec<u64>, delta: f64) {
        let start = set.iter().min().copied().unwrap_or(0);
        let end = set.iter().max().copied().unwrap_or(0);
        assert!(start * 2 >= end);
        let approximation = SumsetIntervalApproximation::<T>::new(start, delta).approximate(&set);
        helpers::test::verify_approximation(&approximation, &naive_sumset(&set), delta, 0);
    }

    fn verify_epsilon_additive_approximation<T: Convoluter>(set: Vec<u64>, epsilon_inv: u64) {
        for &i in &set {
            assert!(epsilon_inv <= i && i < epsilon_inv * 2);
        }
        let approximation =
            SumsetEpsilonAdditiveAproximation::new::<T>(epsilon_inv).approximate(&set);
        helpers::test::verify_approximation(
            &approximation,
            &naive_sumset(&set),
            0.0,
            set.len() as u64,
        );
    }

    #[test]
    fn test_interval_approximation_fft() {
        verify_interval_approximation::<FFT>(vec![5, 6, 7, 8, 9, 10], 0.1);
        verify_interval_approximation::<FFT>(vec![5, 6, 7, 8, 9, 10], 0.01);
        verify_interval_approximation::<FFT>(vec![10, 12, 13, 14, 15, 16, 17, 18, 19, 11], 0.001);
        verify_interval_approximation::<FFT>(vec![10, 12, 13, 14, 15, 16, 17, 18, 19, 11], 0.0001);
        verify_interval_approximation::<FFT>(vec![10, 12, 13, 14, 15, 16, 17, 18, 19, 11], 0.5);
    }
    #[test]
    fn test_interval_approximation_large_fft() {
        verify_interval_approximation::<FFT>(
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
    fn test_interval_epsilon_approximation_fft() {
        verify_epsilon_additive_approximation::<FFT>(vec![6, 7, 8, 9, 10, 11], 6);
        verify_epsilon_additive_approximation::<FFT>((12..24).collect(), 12);
    }
    #[test]
    fn test_interval_approximation_ntt() {
        verify_interval_approximation::<NTT>(vec![5, 6, 7, 8, 9, 10], 0.1);
        verify_interval_approximation::<NTT>(vec![5, 6, 7, 8, 9, 10], 0.01);
        verify_interval_approximation::<NTT>(vec![10, 12, 13, 14, 15, 16, 17, 18, 19, 11], 0.001);
        verify_interval_approximation::<NTT>(vec![10, 12, 13, 14, 15, 16, 17, 18, 19, 11], 0.0001);
        verify_interval_approximation::<NTT>(vec![10, 12, 13, 14, 15, 16, 17, 18, 19, 11], 0.5);
    }
    #[test]
    fn test_interval_approximation_large_ntt() {
        verify_interval_approximation::<NTT>(
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
    fn test_interval_epsilon_approximation_ntt() {
        verify_epsilon_additive_approximation::<NTT>(vec![6, 7, 8, 9, 10, 11], 6);
        verify_epsilon_additive_approximation::<NTT>((12..24).collect(), 12);
    }
}
