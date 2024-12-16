use crate::VALUE_LENGTH;

pub trait Hasher {
    type Hash: Copy;

    fn encode(value: &[u8]) -> Self::Hash;
    fn merkelize(a: Self::Hash, b: Self::Hash) -> Self::Hash;
    fn hash_value(value: &[u8; VALUE_LENGTH]) -> Self::Hash;
    fn zero() -> Self::Hash;
}
