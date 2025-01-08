use crate::{
    hasher::Hasher,
    types::{Key, Value},
    KEY_LENGTH, STEM_SUBTREE_WIDTH,
};

pub enum Node<H: Hasher> {
    Stem {
        depth: u8,
        stem: [u8; KEY_LENGTH - 1],
        values: Box<[Option<Value>; STEM_SUBTREE_WIDTH]>,
    },
    Internal {
        depth: u8,
        left: Option<Box<Self>>,
        right: Option<Box<Self>>,
    },
    Empty,
    #[allow(dead_code)]
    HashedSubtree(H::Hash),
}

impl<H: Hasher> Node<H> {
    pub fn insert(&mut self, key: Key, value: Value) {
        match self {
            Node::Empty => {
                let mut values = [None; STEM_SUBTREE_WIDTH];
                values[key.subindex()] = Some(value);
                *self = Node::Stem {
                    depth: 0,
                    stem: key.stem(),
                    values: Box::new(values),
                };
            }
            Node::Stem {
                stem,
                values,
                depth,
            } => {
                if *stem == key.stem() {
                    values[key.subindex()] = Some(value);
                } else {
                    let existing_stem_node = Node::Stem {
                        depth: *depth + 1,
                        stem: *stem,
                        values: values.clone(),
                    };
                    let existing_bit = Self::get_bit(stem, *depth);
                    let new_bit = Self::get_bit(&key.stem(), *depth);
                    if existing_bit != new_bit {
                        let mut new_stem_values = [None; STEM_SUBTREE_WIDTH];
                        new_stem_values[key.subindex()] = Some(value);
                        let new_stem = Node::Stem {
                            depth: *depth + 1,
                            stem: key.stem(),
                            values: Box::new(new_stem_values),
                        };

                        if existing_bit == 0 {
                            *self = Node::Internal {
                                depth: *depth,
                                left: Some(Box::new(existing_stem_node)),
                                right: Some(Box::new(new_stem)),
                            };
                        } else {
                            *self = Node::Internal {
                                depth: *depth,
                                left: Some(Box::new(new_stem)),
                                right: Some(Box::new(existing_stem_node)),
                            };
                        }
                    } else if existing_bit == 0 {
                        *self = Node::Internal {
                            depth: *depth,
                            left: Some(Box::new(existing_stem_node)),
                            right: None,
                        };
                        if let Node::Internal { left, .. } = self {
                            left.as_mut().unwrap().insert(key, value);
                        }
                    } else {
                        *self = Node::Internal {
                            depth: *depth,
                            left: None,
                            right: Some(Box::new(existing_stem_node)),
                        };
                        if let Node::Internal { right, .. } = self {
                            right.as_mut().unwrap().insert(key, value);
                        }
                    }
                }
            }
            Node::Internal { left, right, depth } => {
                let new_bit = Self::get_bit(&key.stem(), *depth);
                if new_bit == 0 {
                    if let Some(left) = left {
                        left.insert(key, value);
                    } else {
                        let mut new_values = [None; STEM_SUBTREE_WIDTH];
                        new_values[key.subindex()] = Some(value);
                        *left = Some(Box::new(Node::Stem {
                            depth: *depth + 1,
                            stem: key.stem(),
                            values: Box::new(new_values),
                        }));
                    }
                } else if let Some(right) = right {
                    right.insert(key, value);
                } else {
                    let mut new_values = [None; STEM_SUBTREE_WIDTH];
                    new_values[key.subindex()] = Some(value);
                    *right = Some(Box::new(Node::Stem {
                        depth: *depth + 1,
                        stem: key.stem(),
                        values: Box::new(new_values),
                    }));
                }
            }
            Node::HashedSubtree(_) => {
                todo!("handle HashedSubtree in insert")
            }
        }
    }

    fn get_bit(stem: &[u8], depth: u8) -> u8 {
        stem[depth as usize / 8] & (1 << (7 - depth % 8))
    }

    pub fn merkelize(&self) -> H::Hash {
        match self {
            Node::Stem { stem, values, .. } => {
                let mut level: [H::Hash; STEM_SUBTREE_WIDTH] = [H::zero(); STEM_SUBTREE_WIDTH];

                for (i, x) in values.iter().enumerate() {
                    level[i] = match x {
                        Some(val) => H::hash_value(val),
                        None => H::zero(),
                    };
                }
                let mut level_length = level.len();
                while level_length > 1 {
                    for i in (0..level.len()).step_by(2) {
                        level[i] = H::merkelize(level[i], level[i + 1]);
                    }
                    level_length /= 2;
                }
                H::merkelize(H::encode(stem), level[0])
            }
            Node::Internal { left, right, .. } => {
                let left_hash = left.as_ref().map_or(H::zero(), |l| l.merkelize());
                let right_hash = right.as_ref().map_or(H::zero(), |r| r.merkelize());
                H::merkelize(left_hash, right_hash)
            }
            Node::Empty => H::zero(),
            Node::HashedSubtree(hash) => *hash,
        }
    }
}
