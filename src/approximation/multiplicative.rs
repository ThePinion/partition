use std::collections::HashSet;

use crate::helpers::{ceil_div, PowerOfTwoIterator};

use super::additive::AdditiveBoundedMerger;

pub struct MultiplicativeBoundedMerger {
    start: u64,
    length: u64,
    delta: f64,
    t: u64,
}

impl MultiplicativeBoundedMerger {
    pub fn new(start: u64, length: u64, delta: f64, t: u64) -> Self {
        assert!(length <= start);
        assert!(start <= t);
        Self {
            start,
            length,
            delta,
            t,
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
        let merger = AdditiveBoundedMerger::new(self.start, self.length, additive_delta, 6 * r);
        merger
            .merge(a, b)
            .into_iter()
            .filter(|&x| x >= r && x <= self.t)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::verify_approximation;

    use super::*;

    fn verify_merge(a: &[u64], b: &[u64], t: u64, delta: f64) {
        let start = a.iter().chain(b.iter()).min().copied().unwrap_or(0);
        let end = a.iter().chain(b.iter()).max().copied().unwrap_or(0);
        let merger = MultiplicativeBoundedMerger::new(start, end - start, delta, t);
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
    fn test_merge() {
        verify_merge(&[11, 12, 13], &[14, 15, 16], 25, 0.1);
        verify_merge(&[11, 12, 13], &[14, 15, 16], 25, 0.1);
        verify_merge(&[11, 12, 13], &[14, 15, 16], 100, 0.1);
        verify_merge(&[11, 12, 13], &[14, 15, 16], 100, 0.1);
        verify_merge(&[11, 12, 13], &[14, 15, 16], 100, 0.1);
        verify_merge(&[11, 12, 13], &[14, 15, 16], 100, 0.1);

        verify_merge(
            &(1000..1500).into_iter().collect::<Vec<_>>(),
            &(1500..1000).into_iter().collect::<Vec<_>>(),
            2000000,
            0.1,
        )
    }
    #[test]
    fn test_merge_large() {
        verify_merge(
            &(10000..15000).into_iter().collect::<Vec<_>>(),
            &(15000..10000).into_iter().collect::<Vec<_>>(),
            2000000,
            0.1,
        )
    }
    #[test]
    fn test_merge_large_no_approximation() {
        verify_merge(
            &(1500..1800).into_iter().collect::<Vec<_>>(),
            &(1000..1500).into_iter().collect::<Vec<_>>(),
            3000,
            0.00001,
        );
    }
    #[test]
    fn test_merge_small() {
        verify_merge(&[], &[], 25, 0.1);
        verify_merge(&[1], &[], 25, 0.1);
        verify_merge(&[1], &[1], 25, 0.1);
    }
}
