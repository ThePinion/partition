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
    fn fft_size(self) -> usize;
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
        for (i, val) in characteristic.iter().enumerate() {
            if *val {
                out.push(i as u64);
            }
        }
        out
    }

    fn fft_size(self) -> usize {
        self.size
    }
}

#[derive(Debug)]
pub struct Characteristic2d {
    x_size: usize,
    y_size: usize,
}
impl<'a> CharacteristicTrait<(u64, u64)> for &'a Characteristic2d {
    fn encode(self, value: &[(u64, u64)]) -> Vec<bool> {
        let mut encoded = vec![false; self.x_size * self.y_size];
        for (x, y) in value {
            encoded[(x * self.y_size as u64 + y) as usize] = true;
        }
        encoded
    }

    fn decode(self, characteristic: &[bool]) -> Vec<(u64, u64)> {
        let mut out = vec![];
        for (i, val) in characteristic.iter().enumerate() {
            if *val {
                out.push((i as u64 / self.y_size as u64, i as u64 % self.y_size as u64));
            }
        }
        out
    }

    fn fft_size(self) -> usize {
        self.x_size * self.y_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_1d() {
        let characteristic = Characteristic::with_size_1d(10);
        let value = vec![1, 3, 5, 7, 9];
        let encoded = characteristic.encode(&value);
        let decoded = characteristic.decode(&encoded);
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_encode_decode_2d() {
        let characteristic = Characteristic::with_size_2d(3, 4);
        let value = vec![(0, 0), (1, 1), (2, 2), (2, 3)];
        let encoded = characteristic.encode(&value);
        let decoded = characteristic.decode(&encoded);
        assert_eq!(value, decoded);
    }
}
