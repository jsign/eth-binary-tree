use crate::{
    hasher::Hasher,
    types::{Key, Value},
    KEY_LENGTH, STEM_SUBTREE_WIDTH,
};

pub enum Node {
    Stem {
        stem: [u8; KEY_LENGTH - 1],
        values: Box<[Option<Value>; STEM_SUBTREE_WIDTH]>,
    },
    Internal {
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    },
    Empty,
}