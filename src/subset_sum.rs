use crate::{
    characteristic::{Characteristic, CharacteristicTrait as _},
    fft::FFT,
};

pub fn subset_sum(a: &[u64], b: &[u64]) -> Vec<u64> {
    let a_size = (*a.iter().max().unwrap_or(&0u64) as usize) + 1;
    let b_size = (*b.iter().max().unwrap_or(&0u64) as usize) + 1;
    let size = a_size + b_size - 1;
    let encoder = Characteristic::with_size_1d(size);
    let characteristic =
        FFT::new(size).convolute_characteristic_vecs(&encoder.encode(a), &encoder.encode(b));
    encoder.decode(&characteristic)
}
