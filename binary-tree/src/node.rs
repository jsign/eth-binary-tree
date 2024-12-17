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

impl Node {
    pub fn insert(&mut self, key: Key, value: Value, depth: u8) {
        match self {
            Node::Empty => {
                let mut values = [None; STEM_SUBTREE_WIDTH];
                values[key.subindex()] = Some(value);
                *self = Node::Stem {
                    stem: key.stem(),
                    values: Box::new(values),
                };
            }
            Node::Stem { stem, values } => {
                todo!("todo")
            }
            Node::Internal { left, right } => {
                todo!("todo")
            }
        }
    }

    fn get_bit(stem: &[u8], depth: u8) -> u8 {
        stem[depth as usize / 8] & (1 << (7 - depth % 8))
    }
}
