use tree::Tree;

const KEY_LENGTH: usize = 32;
const VALUE_LENGTH: usize = 32;
const STEM_SUBTREE_WIDTH: usize = 256;

pub mod hasher;
mod node;
pub mod tree;
pub mod types;

pub type Blake3Tree = Tree<hasher::Blake3Hasher>;

#[cfg(test)]
mod tests {
    use super::*;
    use hasher::{Blake3Hasher, Hasher};
    use types::{Key, Value};

    // The following are tests mirroring the existing tests
    // in the Python spec implementation (https://github.com/jsign/binary-tree-spec)
    // to check that both implementations match.

    #[test]
    fn empty_tree() {
        let tree = Blake3Tree::new();
        let hash = tree.merkelize();
        assert_eq!(hash, Blake3Hasher::zero());
    }

    #[test]
    fn single_entry() {
        let mut tree = Blake3Tree::new();
        tree.insert(Key([0; 32]), Value([0x01; 32]));
        let hash = tree.merkelize();
        assert_eq!(
            hex::encode(hash),
            "694545468677064fd833cddc8455762fe6b21c6cabe2fc172529e0f573181cd5"
        )
    }
    #[test]
    fn two_entries() {
        let mut tree = Blake3Tree::new();
        tree.insert(Key([0; 32]), Value([0x01; 32]));
        let mut key = [0u8; 32];
        key[0] = 0x80;
        tree.insert(Key(key), Value([0x02; 32]));
        let hash = tree.merkelize();
        assert_eq!(
            hex::encode(hash),
            "85fc622076752a6fcda2c886c18058d639066a83473d9684704b5a29455ed2ed"
        )
    }
}
