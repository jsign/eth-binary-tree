use crate::types::Value;

pub trait Hasher {
    type Hash: Copy;

    fn encode(value: &[u8]) -> Self::Hash;
    fn merkelize(a: Self::Hash, b: Self::Hash) -> Self::Hash;
    fn hash_value(value: &Value) -> Self::Hash;
    fn zero() -> Self::Hash;
}

pub struct Blake3Hasher;

impl Hasher for Blake3Hasher {
    type Hash = [u8; 32];

    fn merkelize(a: Self::Hash, b: Self::Hash) -> Self::Hash {
        let mut input = [0u8; 64];
        input[..32].copy_from_slice(&a);
        input[32..].copy_from_slice(&b);
        if input == [0u8; 64] {
            return [0u8; 32];
        }
        blake3::hash(&input).into()
    }

    fn hash_value(value: &Value) -> Self::Hash {
        blake3::hash(&value.0).into()
    }

    fn zero() -> Self::Hash {
        [0u8; 32]
    }

    fn encode(value: &[u8]) -> Self::Hash {
        if value.len() > 32 {
            panic!("Value too long");
        }
        let mut out = [0u8; 32];
        out[..value.len()].copy_from_slice(value);
        out
    }
}
