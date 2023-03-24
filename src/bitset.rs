use wasm_bindgen::prelude::*;

const fn bin_no(idx: usize) -> usize {
    idx / 32
}
const fn inner_idx(idx: usize) -> usize {
    idx % 32
}

#[wasm_bindgen]
pub struct Bitset {
    bits: Vec<u32>,
}

#[wasm_bindgen]
impl Bitset {
    pub fn with_size(size: usize) -> Self {
        let size = bin_no(size);
        let mut v = Vec::with_capacity(size);
        v.resize(size, 0);
        Bitset { bits: v }
    }
}

#[wasm_bindgen]
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
    pub fn as_ptr(&self) -> *const u32 {
        self.bits.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_no() {
        for i in 0..=128 {
            assert_eq!(i / 32, bin_no(i), "i : {}", i);
        }
    }

    #[test]
    fn test_init() {
        let bs = Bitset::with_size(32);
        assert_eq!(1, bs.bits.len());
        for i in 0..32 {
            assert_eq!(false, bs.get(i), "in index {i}");
        }
        let bs = Bitset::with_size(128);
        assert_eq!(4, bs.bits.len());
        for i in 0..128 {
            assert_eq!(false, bs.get(i), "in index {}", i);
        }
    }
}
