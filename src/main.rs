use partition::approximation::SumsetApproximation;

fn main() {
    dbg!(SumsetApproximation::new(10, 0.1).approximate(&[10, 12, 13, 14]));
}
