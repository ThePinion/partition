use rustfft::{num_complex::Complex, FftPlanner};

pub struct FFT {
    size: usize,
    points_a: Vec<Complex<f32>>,
    points_b: Vec<Complex<f32>>,
    planner: FftPlanner<f32>,
}

impl FFT {
    pub fn new(size: usize) -> Self {
        FFT {
            size,
            points_a: vec![Complex::new(0f32, 0f32); size],
            points_b: vec![Complex::new(0f32, 0f32); size],
            planner: FftPlanner::new(),
        }
    }
    pub fn convolute_characteristic_vecs(&mut self, a: &[bool], b: &[bool]) -> Vec<bool> {
        for (i, val) in a.iter().enumerate() {
            self.points_a[i] = Complex::new(if *val { 1.0 } else { 0.0 }, 0f32);
        }

        for (i, val) in b.iter().enumerate() {
            self.points_b[i] = Complex::new(if *val { 1.0 } else { 0.0 }, 0f32);
        }

        let fft = self.planner.plan_fft_forward(self.size);
        fft.process(&mut self.points_a);
        fft.process(&mut self.points_b);

        for (i, val) in self.points_b.iter().enumerate() {
            self.points_a[i] *= val;
        }

        let fft_inv = self.planner.plan_fft_inverse(self.size);
        fft_inv.process(&mut self.points_a);

        // The 0.9 is a hack to avoid floating point errors, it should theoretically be 1.0
        let product_coefficients: Vec<bool> = self
            .points_a
            .iter()
            .map(|c| c.re >= self.size as f32 * 0.9)
            .collect();
        product_coefficients
    }
}
