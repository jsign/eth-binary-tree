use crate::{KEY_LENGTH, VALUE_LENGTH};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Key(pub [u8; KEY_LENGTH]);

impl Key {
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

impl Distribution<Key> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Key {
        let mut bytes = [0u8; KEY_LENGTH];
        rng.fill(&mut bytes);
        Key(bytes)
    }
}

#[derive(Clone, Copy)]
pub struct Value(pub [u8; VALUE_LENGTH]);

impl Distribution<Value> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Value {
        let mut bytes = [0u8; VALUE_LENGTH];
        rng.fill(&mut bytes);

        Value(bytes)
    }
}
