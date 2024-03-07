pub struct ApproximationVerifier {
    pub epsilon: f32,
    pub delta: f32,
}

impl ApproximationVerifier {
    pub fn new(epsilon: f32, delta: f32) -> Self {
        ApproximationVerifier { epsilon, delta }
    }

    pub fn new_multipicative(epsilon: f32) -> Self {
        Self::new(epsilon, 0f32)
    }

    pub fn new_additive(delta: f32) -> Self {
        Self::new(0f32, delta)
    }

    pub fn verify(&self, original: &[f32], approximation: &[f32]) -> bool {
        todo!()
    }
}