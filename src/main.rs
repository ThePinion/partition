use partition::{approximation::interval::SumsetIntervalApproximation, fft::complex::ComplexFFT};

fn main() {
    dbg!(SumsetIntervalApproximation::<ComplexFFT>::new(10, 0.1).approximate(&[10, 12, 13, 14]));
}
