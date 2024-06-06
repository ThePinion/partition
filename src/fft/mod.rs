pub mod complex;
pub mod number_theoretic;

pub use complex::ComplexFFT as FFT;
pub use number_theoretic::NumberTheoreticTransform as NTT;
pub trait Convoluter {
    fn new(size: usize) -> Self;
    fn convolute_characteristic_vecs(self, a: &[bool], b: &[bool]) -> Vec<bool>;
}
#[cfg(test)]
mod tests {
    use super::{Convoluter, FFT, NTT};

    fn verify_match(a: &[bool], b: &[bool]) {
        let fft = FFT::new(a.len() + b.len()).convolute_characteristic_vecs(a, b);
        let ntt = NTT::new(a.len() + b.len()).convolute_characteristic_vecs(a, b);
        assert_eq!(fft, ntt);
    }

    #[test]
    fn test_fft() {
        verify_match(&[true, false, true, false], &[true, false, false, true]);
    }
}
