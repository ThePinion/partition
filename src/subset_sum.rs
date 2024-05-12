use crate::{
    characteristic::{Characteristic, CharacteristicTrait as _},
    fft::FFT,
};

pub fn subset_sum(a: &[u64], b: &[u64]) -> Vec<u64> {
    let a_size = *a.iter().max().unwrap_or(&0u64) as usize;
    let b_size = *b.iter().max().unwrap_or(&0u64) as usize;
    let size = a_size + b_size + 1;
    let encoder = Characteristic::with_size_1d(size);
    let characteristic =
        FFT::new(size).convolute_characteristic_vecs(&encoder.encode(a), &encoder.encode(b));
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

    let encoder = Characteristic::with_size_2d(x_size, y_size);
    let characteristic = FFT::new(encoder.fft_size())
        .convolute_characteristic_vecs(&encoder.encode(a), &encoder.encode(b));
    encoder.decode(&characteristic)
}
