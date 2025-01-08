use crate::{
    hasher::Hasher,
    node::Node,
    types::{Key, Value},
};

pub struct Tree<H: Hasher> {
    root: Node<H>,
}

impl<H: Hasher> Tree<H> {
    pub fn new() -> Self {
        Self { root: Node::Empty }
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        self.root.insert(key, value);
    }

    pub fn merkelize(&self) -> H::Hash {
        self.root.merkelize()
    }
}

impl<H: Hasher> Default for Tree<H> {
    fn default() -> Self {
        Self::new()
    }
}
