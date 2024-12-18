const KEY_LENGTH: usize = 32;
const VALUE_LENGTH: usize = 32;
const STEM_SUBTREE_WIDTH: usize = 256;

pub mod hasher;
mod node;
pub mod tree;
pub mod types;

#[cfg(test)]
mod tests {
    use hasher::{Blake3Hasher, Hasher};
    use tree::Tree;
    use types::Key;

    use super::*;

    // The following are tests mirroring the existing tests
    // in the Python spec implementation (https://github.com/jsign/binary-tree-spec)
    // to check that both implementations match.

    #[test]
    fn empty_tree() {
        let tree = Tree::new();
        let hash = tree.merkelize::<Blake3Hasher>();
        assert_eq!(hash, Blake3Hasher::zero());
    }

    #[test]
    fn single_entry() {
        let mut tree = Tree::new();
        tree.insert(Key::new(&[0; 32]), [0x01; 32]);
        let hash = tree.merkelize::<Blake3Hasher>();
        assert_eq!(
            hex::encode(hash),
            "694545468677064fd833cddc8455762fe6b21c6cabe2fc172529e0f573181cd5"
        )
    }
    #[test]
    fn two_entries() {
        let mut tree = Tree::new();
        tree.insert(Key::new(&[0; 32]), [0x01; 32]);
        let mut key = [0u8; 32];
        key[0] = 0x80;
        tree.insert(Key::new(&key), [0x02; 32]);
        let hash = tree.merkelize::<Blake3Hasher>();
        assert_eq!(
            hex::encode(hash),
            "85fc622076752a6fcda2c886c18058d639066a83473d9684704b5a29455ed2ed"
        )
    }
}
