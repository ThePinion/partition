use std::collections::HashSet;

use crate::{
    characteristic::{Characteristic, CharacteristicTrait as _},
    fft::Convoluter,
};

pub fn subset_sum<T: Convoluter>(a: &[u64], b: &[u64]) -> Vec<u64> {
    let a_size = *a.iter().max().unwrap_or(&0u64) as usize;
    let b_size = *b.iter().max().unwrap_or(&0u64) as usize;
    let size = a_size + b_size + 1;
    bounded_subset_sum::<T>(a, b, size)
}

pub fn bounded_subset_sum<T: Convoluter>(a: &[u64], b: &[u64], bound: usize) -> Vec<u64> {
    if a.len() * b.len() < 1000 {
        return naive_sumset_sum(a, b);
    }
    let encoder = Characteristic::with_size_1d(bound);
    let characteristic =
        T::new(bound).convolute_characteristic_vecs(&encoder.encode(a), &encoder.encode(b));
    encoder.decode(&characteristic)
}

pub fn subset_sum_2d<T: Convoluter>(a: &[(u64, u64)], b: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let (a_x_size, a_y_size) = a
        .iter()
        .fold((0, 0), |acc, f| (acc.0.max(f.0), acc.1.max(f.1)));

    let (b_x_size, b_y_size) = b
        .iter()
        .fold((0, 0), |acc, f| (acc.0.max(f.0), acc.1.max(f.1)));

    let x_size = (a_x_size + b_x_size + 1) as usize;
    let y_size = (a_y_size + b_y_size + 1) as usize;
    bounded_subset_sum_2d::<T>(a, b, x_size, y_size)
}

pub fn bounded_subset_sum_2d<T: Convoluter>(
    a: &[(u64, u64)],
    b: &[(u64, u64)],
    x_size: usize,
    y_size: usize,
) -> Vec<(u64, u64)> {
    if a.len() * b.len() < 1000 {
        return naive_sumset_sum_2d(a, b);
    }
    let encoder = Characteristic::with_size_2d(x_size, y_size);
    let characteristic = T::new(encoder.fft_size())
        .convolute_characteristic_vecs(&encoder.encode(a), &encoder.encode(b));
    encoder.decode(&characteristic)
}

fn naive_sumset_sum(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut result = HashSet::new();
    for x in a {
        for y in b {
            result.insert(x + y);
        }
    }
    result.into_iter().collect()
}

fn naive_sumset_sum_2d(a: &[(u64, u64)], b: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut result = HashSet::new();
    for x in a {
        for y in b {
            result.insert((x.0 + y.0, x.1 + y.1));
        }
    }
    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::fft::{FFT, NTT};

    use super::*;

    fn test_1d<T: Convoluter>(a: &[u64], b: &[u64]) {
        let result = HashSet::from_iter(subset_sum::<T>(a, b));
        let mut expected = HashSet::new();
        for i in a {
            for j in b {
                expected.insert(i + j);
            }
        }
        assert_eq!(result, expected);
    }

    fn test_2d<T: Convoluter>(a: &[(u64, u64)], b: &[(u64, u64)]) {
        let result = HashSet::from_iter(subset_sum_2d::<T>(a, b));
        let mut expected = HashSet::new();
        for (i, j) in a {
            for (k, l) in b {
                expected.insert((i + k, j + l));
            }
        }
        assert_eq!(result, expected);
    }

    #[test]
    fn test_subset_sum_fft() {
        test_1d::<FFT>(&[1, 2], &[1, 100]);
        test_1d::<FFT>(&[1, 2], &[]);
        test_1d::<FFT>(&[], &[1, 100]);
        test_1d::<FFT>(&[], &[]);
        test_1d::<FFT>(&[1, 2, 3], &[1, 2, 3]);
        test_1d::<FFT>(
            &(0..100).collect::<Vec<_>>(),
            &(2..20000).collect::<Vec<_>>(),
        )
    }

    #[test]
    fn test_subset_sum_ntt() {
        test_1d::<NTT>(&[1, 2], &[1, 100]);
        test_1d::<NTT>(&[1, 2], &[]);
        test_1d::<NTT>(&[], &[1, 100]);
        test_1d::<NTT>(&[], &[]);
        test_1d::<NTT>(&[1, 2, 3], &[1, 2, 3]);
        test_1d::<NTT>(
            &(0..100).collect::<Vec<_>>(),
            &(2..20000).collect::<Vec<_>>(),
        )
    }

    #[test]
    fn test_subset_sum_2d_fft() {
        test_2d::<FFT>(&[(1, 0), (2, 1)], &[(1, 10), (100, 20)]);
        test_2d::<FFT>(&[(1, 0), (2, 1)], &[]);
        test_2d::<FFT>(&[], &[(1, 10), (100, 20)]);
        test_2d::<FFT>(&[], &[]);
        test_2d::<FFT>(
            &(0..100).map(|a| (a, a + 6)).collect::<Vec<_>>(),
            &(200..300).map(|a| (a, a + 600)).collect::<Vec<_>>(),
        )
    }

    #[test]
    fn test_subset_sum_2d_ntt() {
        test_2d::<NTT>(&[(1, 0), (2, 1)], &[(1, 10), (100, 20)]);
        test_2d::<NTT>(&[(1, 0), (2, 1)], &[]);
        test_2d::<NTT>(&[], &[(1, 10), (100, 20)]);
        test_2d::<NTT>(&[], &[]);
        test_2d::<NTT>(
            &(0..100).map(|a| (a, a + 6)).collect::<Vec<_>>(),
            &(200..300).map(|a| (a, a + 600)).collect::<Vec<_>>(),
        )
    }
}
