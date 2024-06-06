pub mod complex;
pub mod number_theoretic;

pub type FFT = complex::ComplexFFT;

pub trait FFTConvoluter {
    fn new(size: usize) -> Self;
    fn convolute_characteristic_vecs(&mut self, a: &[bool], b: &[bool]) -> Vec<bool>;
}
