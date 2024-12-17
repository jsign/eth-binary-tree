use crate::{
    hasher::Hasher,
    node::Node,
    types::{Key, Value},
};

pub struct Tree {
    root: Node,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: Node::Empty }
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        self.root.insert(key, value, 0);
    }

    pub fn merkelize<H: Hasher>(&self) -> H::Hash {
        self.root.merkelize::<H>()
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}
