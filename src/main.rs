use partition::{approximation::interval::SumsetIntervalApproximation, fft::FFT};

fn main() {
    dbg!(SumsetIntervalApproximation::<FFT>::new(10, 0.1).approximate(&[10, 12, 13, 14]));
}
