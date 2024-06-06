use std::{collections::HashSet, marker::PhantomData};

use crate::{
    fft::Convoluter,
    helpers::{ceil_div, PowerOfTwoIterator},
};

use super::AdditiveBoundedMerger;

pub struct MultiplicativeBoundedMerger<T: Convoluter> {
    start: u64,
    length: u64,
    delta: f64,
    t: u64,
    _phantom: PhantomData<T>,
}

impl<T: Convoluter> MultiplicativeBoundedMerger<T> {
    pub fn new(start: u64, length: u64, delta: f64, t: u64) -> Self {
        assert!(length <= start);
        assert!(start <= t);
        Self {
            start,
            length,
            delta,
            t,
            _phantom: PhantomData,
        }
    }
    pub fn merge(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut result = HashSet::new();
        for r in PowerOfTwoIterator::new(ceil_div(self.start, 6), self.t) {
            let merged = self.merge_interval(a, b, r);
            result.extend(merged);
        }
        result.into_iter().collect()
    }
    fn merge_interval(&self, a: &[u64], b: &[u64], r: u64) -> Vec<u64> {
        let additive_delta = (self.delta * r as f64).ceil() as u64;
        let merger =
            AdditiveBoundedMerger::<T>::new(self.start, self.length, additive_delta, 6 * r);
        merger
            .merge(a, b)
            .into_iter()
            .filter(|&x| x >= r && x <= self.t)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        fft::{FFT, NTT},
        helpers::test::verify_approximation,
    };

    use super::*;

    fn verify_multiplicative_merge<T: Convoluter>(a: &[u64], b: &[u64], t: u64, delta: f64) {
        let start = a.iter().chain(b.iter()).min().copied().unwrap_or(0);
        let end = a.iter().chain(b.iter()).max().copied().unwrap_or(0);
        let merger = MultiplicativeBoundedMerger::<T>::new(start, end - start, delta, t);
        let merged = merger.merge(a, b);
        let mut expected = vec![];
        for i in a {
            for j in b {
                if i + j <= t {
                    expected.push(i + j);
                }
            }
        }
        verify_approximation(&merged, &expected, delta, 0)
    }

    #[test]
    fn test_multiplicative_merge_fft() {
        verify_multiplicative_merge::<FFT>(&[11, 12, 13], &[14, 15, 16], 25, 0.1);
        verify_multiplicative_merge::<FFT>(&[11, 12, 13], &[14, 15, 16], 25, 0.1);
        verify_multiplicative_merge::<FFT>(&[11, 12, 13], &[14, 15, 16], 100, 0.1);
        verify_multiplicative_merge::<FFT>(&[11, 12, 13], &[14, 15, 16], 100, 0.1);
        verify_multiplicative_merge::<FFT>(&[11, 12, 13], &[14, 15, 16], 100, 0.1);
        verify_multiplicative_merge::<FFT>(&[11, 12, 13], &[14, 15, 16], 100, 0.1);
    }
    #[test]
    fn test_multiplicative_merge_large_fft() {
        verify_multiplicative_merge::<FFT>(
            &(10000..15000).collect::<Vec<_>>(),
            &(10000..11000).collect::<Vec<_>>(),
            2000000,
            0.3,
        )
    }
    #[test]
    fn test_multiplicative_merge_large_barely_approximate_fft() {
        verify_multiplicative_merge::<FFT>(
            &(1500..1800).collect::<Vec<_>>(),
            &(1000..1500).collect::<Vec<_>>(),
            3000,
            0.00001,
        );
    }
    #[test]
    fn test_multiplicative_merge_small_fft() {
        verify_multiplicative_merge::<FFT>(&[], &[], 25, 0.1);
        verify_multiplicative_merge::<FFT>(&[1], &[], 25, 0.1);
        verify_multiplicative_merge::<FFT>(&[1], &[1], 25, 0.1);
    }

    #[test]
    fn test_multiplicative_merge_ntt() {
        verify_multiplicative_merge::<NTT>(&[11, 12, 13], &[14, 15, 16], 25, 0.1);
        verify_multiplicative_merge::<NTT>(&[11, 12, 13], &[14, 15, 16], 25, 0.1);
        verify_multiplicative_merge::<NTT>(&[11, 12, 13], &[14, 15, 16], 100, 0.1);
        verify_multiplicative_merge::<NTT>(&[11, 12, 13], &[14, 15, 16], 100, 0.1);
        verify_multiplicative_merge::<NTT>(&[11, 12, 13], &[14, 15, 16], 100, 0.1);
        verify_multiplicative_merge::<NTT>(&[11, 12, 13], &[14, 15, 16], 100, 0.1);
    }
    #[test]
    fn test_multiplicative_merge_large_ntt() {
        verify_multiplicative_merge::<NTT>(
            &(10000..15000).collect::<Vec<_>>(),
            &(10000..11000).collect::<Vec<_>>(),
            2000000,
            0.3,
        )
    }
    #[test]
    fn test_multiplicative_merge_large_barely_approximate_ntt() {
        verify_multiplicative_merge::<NTT>(
            &(1500..1800).collect::<Vec<_>>(),
            &(1000..1500).collect::<Vec<_>>(),
            3000,
            0.00001,
        );
    }
    #[test]
    fn test_multiplicative_merge_small_ntt() {
        verify_multiplicative_merge::<NTT>(&[], &[], 25, 0.1);
        verify_multiplicative_merge::<NTT>(&[1], &[], 25, 0.1);
        verify_multiplicative_merge::<NTT>(&[1], &[1], 25, 0.1);
    }
}
