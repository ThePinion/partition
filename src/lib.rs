pub mod approximation;
pub mod characteristic;
pub mod fft;
pub mod helpers;
pub mod subset_sum;

pub use approximation::approximate_sumset;
pub use fft::{Convoluter, FFT, NTT};

pub fn approximate_partition<T: Convoluter>(input: &[u16], epsilon: f64) -> f64 {
    let approximation = approximate_sumset::<T>(input, epsilon);
    let sigma: u64 = input.iter().copied().map(u64::from).sum();
    let t = (sigma / 2) as f64;
    let a = approximation
        .into_iter()
        .filter(|&x| x as f64 <= t)
        .max()
        .unwrap_or(0) as f64;
    let compilment = t * (1.0 - epsilon / 2.0);
    a.min(compilment)
}

#[cfg(test)]
mod tests {
    use crate::{helpers::test::naive_sumset, Convoluter, FFT, NTT};

    fn validate_partition_approximation<T: Convoluter>(input: &[u16], epsilon: f64) {
        let approximation = super::approximate_partition::<T>(input, epsilon);
        let t: u64 = input.iter().copied().map(u64::from).sum::<u64>() / 2;
        let opt = naive_sumset(&input.iter().copied().map(u64::from).collect::<Vec<_>>())
            .into_iter()
            .filter(|&x| x <= t)
            .max()
            .unwrap_or(0);
        assert!(
            (opt as f64 - approximation) <= epsilon * t as f64,
            "{}, {}, {}",
            opt,
            approximation,
            epsilon
        );
    }

    fn validate_known_partition_approximation<T: Convoluter>(
        input: &[u16],
        epsilon: f64,
        opt: u64,
    ) {
        let approximation = super::approximate_partition::<T>(input, epsilon);
        let t: u64 = input.iter().copied().map(u64::from).sum::<u64>() / 2;
        assert!(
            (opt as f64 - approximation) <= epsilon * t as f64,
            "{}, {}, {}",
            opt,
            approximation,
            epsilon
        );
    }

    #[test]
    fn test_partition_fft() {
        let input = [
            1001, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 1000, 1001, 1002, 1003, 5,
        ]
        .to_vec();
        validate_partition_approximation::<FFT>(&input, 0.01);
    }
    #[test]
    fn test_partition_ntt() {
        let input = [
            1001, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 1000, 1001, 1002, 1003, 5,
        ]
        .to_vec();
        validate_partition_approximation::<NTT>(&input, 0.01);
    }

    #[test]
    fn test_partition_empty() {
        let input = [].to_vec();
        validate_partition_approximation::<NTT>(&input, 0.01);
        validate_partition_approximation::<FFT>(&input, 0.01);
    }

    #[test]
    fn test_partition_single() {
        let input = [u16::MAX].to_vec();
        validate_partition_approximation::<NTT>(&input, 0.01);
        validate_partition_approximation::<FFT>(&input, 0.01);
    }

    #[test]
    fn test_partition_large_numbers() {
        let input = (0..10)
            .map(|x| u16::MAX - x * x * x * x)
            .collect::<Vec<_>>();
        validate_partition_approximation::<NTT>(&input, 0.01);
        validate_partition_approximation::<FFT>(&input, 0.01);
    }

    #[test]
    fn test_partition_large_numbers_precise_approx() {
        let input = (0..10)
            .map(|x| u16::MAX - x * x * x * x)
            .collect::<Vec<_>>();
        validate_partition_approximation::<NTT>(&input, 0.0001);
        validate_partition_approximation::<FFT>(&input, 0.01);
    }

    #[test]
    fn test_partition_loose_approx() {
        let input = (0..10)
            .map(|x| u16::MAX - x * x * x * x)
            .collect::<Vec<_>>();
        validate_partition_approximation::<NTT>(&input, 0.5);
        validate_partition_approximation::<FFT>(&input, 0.01);
    }

    #[test]
    fn test_partition_known_large_1_ntt() {
        let input = vec![1000; 5001];
        validate_known_partition_approximation::<NTT>(&input, 0.01, 1000 * 2500);
        validate_known_partition_approximation::<NTT>(&input[0..5000], 0.01, 1000 * 2500);
    }

    #[test]
    fn test_partition_known_large_1_fft() {
        let input = vec![1000; 5001];
        validate_known_partition_approximation::<FFT>(&input, 0.01, 1000 * 2500);
        validate_known_partition_approximation::<FFT>(&input[0..5000], 0.01, 1000 * 2500);
    }

    #[test]
    fn test_partition_known_large_2_ntt() {
        let input = vec![2; 100001];
        validate_known_partition_approximation::<NTT>(&input, 0.01, 100000);
    }
    #[test]
    fn test_partition_known_large_3_ntt() {
        let input = vec![2; 100000];
        validate_known_partition_approximation::<NTT>(&input, 0.01, 100000);
    }
    #[test]
    fn test_partition_known_large_2_fft() {
        let input = vec![2; 100001];
        validate_known_partition_approximation::<FFT>(&input, 0.01, 100000);
    }
    #[test]
    fn test_partition_known_large_3_fft() {
        let input = vec![2; 100000];
        validate_known_partition_approximation::<FFT>(&input[0..100000], 0.01, 100000);
    }
}
