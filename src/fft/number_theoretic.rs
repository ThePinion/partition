use concrete_ntt::prime64::Plan;

use super::Convoluter;

pub struct NumberTheoreticTransform {
    size: usize,
    points_a: Vec<u64>,
    points_b: Vec<u64>,
    planner: Plan,
}

// const PRIME: u64 = 9807971461541688691;
const PRIME: u64 = 1073479681;

impl Convoluter for NumberTheoreticTransform {
    fn new(size: usize) -> Self {
        let pow_2_size = size.next_power_of_two().max(16);
        NumberTheoreticTransform {
            planner: Plan::try_new(pow_2_size, PRIME).unwrap(),
            size,
            points_a: vec![0; pow_2_size],
            points_b: vec![0; pow_2_size],
        }
    }
    fn convolute_characteristic_vecs(mut self, a: &[bool], b: &[bool]) -> Vec<bool> {
        for (i, val) in a.iter().enumerate() {
            self.points_a[i] = if *val { 1 } else { 0 };
        }
        for (i, val) in b.iter().enumerate() {
            self.points_b[i] = if *val { 1 } else { 0 };
        }
        self.planner.fwd(&mut self.points_a);
        self.planner.fwd(&mut self.points_b);
        self.planner
            .mul_assign_normalize(&mut self.points_a, &self.points_b);
        self.planner.inv(&mut self.points_a);
        self.points_a
            .into_iter()
            .take(self.size)
            .map(|x| x != 0)
            .collect()
    }
}
