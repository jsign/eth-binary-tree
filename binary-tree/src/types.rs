use crate::KEY_LENGTH;

pub struct Key([u8; KEY_LENGTH]);

impl Key {
    pub fn new(key: &[u8; KEY_LENGTH]) -> Self {
        Key(*key)
    }
    pub fn ith_bit(&self, i: usize) -> bool {
        self.0[i / 8] & (1 << (i % 8)) != 0
    }

    pub fn stem(&self) -> [u8; KEY_LENGTH - 1] {
        let mut stem = [0; KEY_LENGTH - 1];
        stem.copy_from_slice(&self.0[..KEY_LENGTH - 1]);
        stem
    }

    pub fn subindex(&self) -> usize {
        self.0[KEY_LENGTH - 1] as usize
    }
}

pub type Value = [u8; KEY_LENGTH];
