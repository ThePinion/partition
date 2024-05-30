use crate::helpers::*;
use crate::subset_sum::{bounded_subset_sum, bounded_subset_sum_2d};

#[derive(Debug, Clone, Copy)]
pub struct AdditiveBoundedMerger {
    start: u64,
    length: u64,
    t: u64,
    base: u64,
    is_2d: bool,
}

impl AdditiveBoundedMerger {
    pub fn new(start: u64, length: u64, delta: u64, t: u64) -> Self {
        assert!(length <= start);
        assert!(start <= t);
        Self {
            start,
            length,
            base: ceil_div(delta, 2),
            t,
            is_2d: fft2d_complexity(start, length, t, delta) < fft1d_complexity(t, delta),
        }
    }
    pub fn merge(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        if self.is_2d {
            self.merge_2d(a, b)
        } else {
            self.merge_1d(a, b)
        }
    }
    pub fn merge_1d(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        let based_merged = bounded_subset_sum(
            &self.based_1d_representation(a),
            &self.based_1d_representation(b),
            ceil_div(self.t, self.base) as usize * 2_usize,
        );
        self.unbased_1d_representation(&based_merged)
            .into_iter()
            .filter(|&x| x <= self.t)
            .collect()
    }
    pub fn merge_2d(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        let based_merged = bounded_subset_sum_2d(
            &self.based_2d_representation(a),
            &self.based_2d_representation(b),
            ceil_div(self.t, self.start) as usize + 1,
            ceil_div(self.t * self.length, self.start * self.base).max(1) as usize,
        );
        self.unbased_2d_representation(&based_merged)
            .into_iter()
            .filter(|&x| x <= self.t)
            .collect()
    }

    fn based_1d_representation(&self, a: &[u64]) -> Vec<u64> {
        a.iter()
            .filter_map(|x| {
                if *x < self.t {
                    Some(x / self.base)
                } else {
                    None
                }
            })
            .collect()
    }
    fn unbased_1d_representation(&self, a: &[u64]) -> Vec<u64> {
        a.iter().map(|x| x * self.base).collect()
    }
    fn based_2d_representation(&self, a: &[u64]) -> Vec<(u64, u64)> {
        a.iter()
            .filter(|&&n| n <= self.t)
            .map(|n| {
                let x = n / self.start;
                let y = (n - (x * self.start)) / self.base;
                (x, y)
            })
            .collect()
    }
    fn unbased_2d_representation(&self, a: &[(u64, u64)]) -> Vec<u64> {
        a.iter()
            .map(|(x, y)| x * self.start + y * self.base)
            .collect()
    }
}

fn fft1d_complexity(t: u64, delta: u64) -> u64 {
    ceil_div(t, delta)
}
fn fft2d_complexity(start: u64, size: u64, t: u64, delta: u64) -> u64 {
    ceil_div(size * t, start * delta) * t / start
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_merge(a: &[u64], b: &[u64], t: u64, delta: u64) {
        let start = a.iter().chain(b.iter()).min().copied().unwrap_or(0);
        let end = a.iter().chain(b.iter()).max().copied().unwrap_or(0);
        let merger = AdditiveBoundedMerger::new(start, end - start, delta, t);
        let merged = merger.merge(a, b);
        let mut expected = vec![];
        for i in a {
            for j in b {
                if i + j <= t {
                    expected.push(i + j);
                }
            }
        }
        verify_approximation(&merged, &expected, 0f64, delta)
    }

    #[test]
    fn test_merge() {
        verify_merge(&[10, 12, 13], &[14, 15, 16], 10, 2);
        verify_merge(&[10, 12, 13], &[14, 15, 16], 10, 1);
        verify_merge(&[10, 12, 13], &[14, 15, 16], 100, 3);
        verify_merge(&[10, 12, 13], &[14, 15, 16], 100, 4);
        verify_merge(&[10, 12, 13], &[14, 15, 16], 100, 5);
        verify_merge(&[10, 12, 13], &[14, 15, 16], 100, 6);

        verify_merge(
            &(1000..1500).into_iter().collect::<Vec<_>>(),
            &(1500..1000).into_iter().collect::<Vec<_>>(),
            2000000,
            100,
        )
    }
    #[test]
    fn test_merge_large() {
        verify_merge(
            &(10000..15000).into_iter().collect::<Vec<_>>(),
            &(15000..10000).into_iter().collect::<Vec<_>>(),
            2000000,
            100,
        )
    }
    #[test]
    fn test_merge_large_no_approximation() {
        verify_merge(
            &(1500..1800).into_iter().collect::<Vec<_>>(),
            &(1000..1500).into_iter().collect::<Vec<_>>(),
            3000,
            1,
        );
    }
}
