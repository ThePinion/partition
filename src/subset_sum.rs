use crate::{
    characteristic::{Characteristic, CharacteristicTrait as _},
    fft::FFT,
};

pub fn subset_sum(a: &[u64], b: &[u64]) -> Vec<u64> {
    let a_size = *a.iter().max().unwrap_or(&0u64) as usize;
    let b_size = *b.iter().max().unwrap_or(&0u64) as usize;
    let size = a_size + b_size + 1;
    bounded_subset_sum(a, b, size)
}

pub fn bounded_subset_sum(a: &[u64], b: &[u64], bound: usize) -> Vec<u64> {
    let encoder = Characteristic::with_size_1d(bound);
    let characteristic =
        FFT::new(bound).convolute_characteristic_vecs(&encoder.encode(a), &encoder.encode(b));
    encoder.decode(&characteristic)
}

pub fn subset_sum_2d(a: &[(u64, u64)], b: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let (a_x_size, a_y_size) = a
        .iter()
        .fold((0, 0), |acc, f| (acc.0.max(f.0), acc.1.max(f.1)));

    let (b_x_size, b_y_size) = b
        .iter()
        .fold((0, 0), |acc, f| (acc.0.max(f.0), acc.1.max(f.1)));

    let x_size = (a_x_size + b_x_size + 1) as usize;
    let y_size = (a_y_size + b_y_size + 1) as usize;
    bounded_subset_sum_2d(a, b, x_size, y_size)
}

pub fn bounded_subset_sum_2d(
    a: &[(u64, u64)],
    b: &[(u64, u64)],
    x_size: usize,
    y_size: usize,
) -> Vec<(u64, u64)> {
    let encoder = Characteristic::with_size_2d(x_size, y_size);
    let characteristic = FFT::new(encoder.fft_size())
        .convolute_characteristic_vecs(&encoder.encode(a), &encoder.encode(b));
    encoder.decode(&characteristic)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn test_1d(a: &[u64], b: &[u64]) {
        let result = HashSet::from_iter(subset_sum(a, b));
        let mut expected = HashSet::new();
        for i in a {
            for j in b {
                expected.insert(i + j);
            }
        }
        assert_eq!(result, expected);
    }

    fn test_2d(a: &[(u64, u64)], b: &[(u64, u64)]) {
        let result = HashSet::from_iter(subset_sum_2d(a, b));
        let mut expected = HashSet::new();
        for (i, j) in a {
            for (k, l) in b {
                expected.insert((i + k, j + l));
            }
        }
        assert_eq!(result, expected);
    }

    #[test]
    fn test_subset_sum() {
        test_1d(&[1, 2], &[1, 100]);
        test_1d(&[1, 2], &[]);
        test_1d(&[], &[1, 100]);
        test_1d(&[], &[]);
        test_1d(&[1, 2, 3], &[1, 2, 3]);
        test_1d(
            &(0..100).into_iter().collect::<Vec<_>>(),
            &(2..20000).into_iter().collect::<Vec<_>>(),
        )
    }

    #[test]
    fn test_subset_sum_2d() {
        test_2d(&[(1, 0), (2, 1)], &[(1, 10), (100, 20)]);
        test_2d(&[(1, 0), (2, 1)], &[]);
        test_2d(&[], &[(1, 10), (100, 20)]);
        test_2d(&[], &[]);
        test_2d(
            &(0..100).into_iter().map(|a| (a, a + 6)).collect::<Vec<_>>(),
            &(200..300)
                .map(|a| (a, a + 600))
                .into_iter()
                .collect::<Vec<_>>(),
        )
    }
}
