use rustfft::{num_complex::Complex, FftPlanner};

fn fft_polynomials(a: &[bool], b: &[bool]) -> Vec<bool> {
    let size = a.len() + b.len() - 1;
    let mut planner = FftPlanner::new();

    let mut points_a = vec![Complex::new(0f32, 0f32); size];
    let mut points_b = vec![Complex::new(0f32, 0f32); size];

    for i in 0..a.len() {
        points_a[i] = Complex::new(if a[i] { 1.0 } else { 0.0 }, 0f32);
    }

    for i in 0..b.len() {
        points_b[i] = Complex::new(if b[i] { 1.0 } else { 0.0 }, 0f32);
    }

    let fft = planner.plan_fft_forward(size);
    fft.process(&mut points_a);
    fft.process(&mut points_b);

    for i in 0..size {
        points_a[i] *= points_b[i];
    }

    let fft_inv = planner.plan_fft_inverse(size);
    fft_inv.process(&mut points_a);

    // Extract coefficients of the product polynomial
    let product_coefficients: Vec<bool> =
        points_a.iter().map(|c| c.re >= size as f32 * 0.9).collect();
    product_coefficients
}

fn characteristic_vector(a: &[u64]) -> Vec<bool> {
    let mut v = vec![false; (*a.iter().max().unwrap_or(&0u64) as usize) + 1];
    for i in a {
        v[*i as usize] = true;
    }
    v
}

fn characteristic_vector_inverse(a: &[bool]) -> Vec<u64> {
    let mut out = vec![];
    for i in 0..a.len() {
        if a[i] {
            out.push(i as u64);
        }
    }
    out
}

fn subset_sum(a: &[u64], b: &[u64]) -> Vec<u64> {
    let char_a = characteristic_vector(a);
    let char_b = characteristic_vector(b);
    let char_c = fft_polynomials(&char_a, &char_b);
    characteristic_vector_inverse(&char_c)
}

fn main() {
    dbg!(subset_sum(&[1, 2], &[1, 1000000]));
}
