pub enum Characteristic {}

impl Characteristic {
    pub fn with_size_1d(size: usize) -> Characteristic1d {
        Characteristic1d { size }
    }
    pub fn with_size_2d(x_size: usize, y_size: usize) -> Characteristic2d {
        Characteristic2d { x_size, y_size }
    }
}

pub trait CharacteristicTrait<T> {
    fn encode(self, value: &[T]) -> Vec<bool>;
    fn decode(self, characteristic: &[bool]) -> Vec<T>;
}

pub struct Characteristic1d {
    size: usize,
}

impl<'a> CharacteristicTrait<u64> for &'a Characteristic1d {
    fn encode(self, value: &[u64]) -> Vec<bool> {
        let mut encoded = vec![false; self.size];
        for i in value {
            encoded[*i as usize] = true;
        }
        encoded
    }

    fn decode(self, characteristic: &[bool]) -> Vec<u64> {
        let mut out = vec![];
        for i in 0..characteristic.len() {
            if characteristic[i] {
                out.push(i as u64);
            }
        }
        out
    }
}

pub struct Characteristic2d {
    x_size: usize,
    y_size: usize,
}
impl<'a> CharacteristicTrait<(u64, u64)> for &'a Characteristic2d {
    fn encode(self, value: &[(u64, u64)]) -> Vec<bool> {
        todo!()
    }

    fn decode(self, characteristic: &[bool]) -> Vec<(u64, u64)> {
        todo!()
    }
}

// fn characteristic_vector_2d(a: &[(u64, u64)]) -> Vec<bool> {
//     let max_x = a.iter().map(|(x, _)| x).max().unwrap_or(&0u64);
//     let max_y = a.iter().map(|(_, y)| y).max().unwrap_or(&0u64);
//     for i in a {
//         v[*i as usize] = true;
//     }
//     v
// }

// fn characteristic_vector_2d_inverse(a: &[bool]) -> Vec<u64> {
//     let mut out = vec![];
//     for i in 0..a.len() {
//         if a[i] {
//             out.push(i as u64);
//         }
//     }
//     out
// }
