fn bin_no(idx: usize) -> usize {
    idx / (usize::BITS as usize)
}
fn inner_idx(idx: usize) -> usize {
    idx % (usize::BITS as usize)
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Bitset {
    bits: Vec<usize>,
}

impl Bitset {
    pub fn with_size(size: usize) -> Self {
        let size = bin_no(size) + 1;
        let v = vec![0; size];
        Bitset { bits: v }
    }
    pub fn from_indices(indices: &[usize]) -> Self {
        let size = bin_no(indices.len()) + 1;
        let mut v = vec![0; size];
        for i in indices.iter().cloned() {
            *v.get_mut(bin_no(i)).expect("Index Out of Bounds") |= 1 << inner_idx(i);
        }
        Bitset { bits: v }
    }
}

impl Bitset {
    pub fn get(&self, idx: usize) -> bool {
        self.bits.get(bin_no(idx)).expect("Index Out of Bounds") >> inner_idx(idx) & 0b01 == 1
    }
    pub fn set(&mut self, idx: usize) {
        *self.bits.get_mut(bin_no(idx)).expect("Index Out of Bounds") |= 1 << inner_idx(idx);
    }
    pub fn reset(&mut self, idx: usize) {
        *self.bits.get_mut(bin_no(idx)).expect("Index Out of Bounds") &= !(1 << inner_idx(idx));
    }
    pub fn set_to(&mut self, idx: usize, to: bool) {
        if to {
            self.set(idx);
        } else {
            self.reset(idx);
        }
    }
    /// JS에게 opaque handle을 제공해 주기 위한 raw pointer 제공자.
    /// JS 단에서 매번 get, set을 호출할 수도 있겠지만 잦은 wasm 함수 호출은 병목현상이 발생할 수 있다고 하대?
    pub fn as_ptr(&self) -> *const usize {
        self.bits.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_no() {
        for i in 0..=128 {
            assert_eq!(i / (usize::BITS as usize), bin_no(i), "i : {}", i);
        }
    }

    #[test]
    fn test_init() {
        let bs = Bitset::with_size(32);
        for i in 0..32 {
            assert_eq!(false, bs.get(i), "in index {i}");
        }
        let bs = Bitset::with_size(128);
        for i in 0..128 {
            assert_eq!(false, bs.get(i), "in index {}", i);
        }
    }

    #[test]
    fn test_set_get() {
        let size = 128;
        let mut bs = Bitset::with_size(size);
        for i in 0..size {
            bs.set(i);
            assert!(bs.get(i));
        }
        for i in (0..size).rev() {
            bs.reset(i);
            assert!(!bs.get(i));
        }
    }
}
