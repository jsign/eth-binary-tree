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
                if *stem == key.stem() {
                    values[key.subindex()] = Some(value);
                } else {
                    let existing_stem = Node::Stem {
                        stem: *stem,
                        values: values.clone(),
                    };
                    let existing_bit = Node::get_bit(stem, depth);
                    let new_bit = Node::get_bit(&key.stem(), depth);
                    if existing_bit != new_bit {
                        let mut new_stem_values = [None; STEM_SUBTREE_WIDTH];
                        new_stem_values[key.subindex()] = Some(value);
                        let new_stem = Node::Stem {
                            stem: key.stem(),
                            values: Box::new(new_stem_values),
                        };

                        if existing_bit == 0 {
                            *self = Node::Internal {
                                left: Some(Box::new(existing_stem)),
                                right: Some(Box::new(new_stem)),
                            };
                        } else {
                            *self = Node::Internal {
                                left: Some(Box::new(new_stem)),
                                right: Some(Box::new(existing_stem)),
                            };
                        }
                    } else if existing_bit == 0 {
                        *self = Node::Internal {
                            left: Some(Box::new(existing_stem)),
                            right: None,
                        };
                        if let Node::Internal { left, .. } = self {
                            left.as_mut().unwrap().insert(key, value, depth + 1);
                        }
                    } else {
                        *self = Node::Internal {
                            left: None,
                            right: Some(Box::new(existing_stem)),
                        };
                        if let Node::Internal { right, .. } = self {
                            right.as_mut().unwrap().insert(key, value, depth + 1);
                        }
                    }
                }
            }
            Node::Internal { left, right } => {
                let new_bit = Self::get_bit(&key.stem(), depth);
                if new_bit == 0 {
                    if let Some(left) = left {
                        left.insert(key, value, depth + 1);
                    } else {
                        let mut new_values = [None; STEM_SUBTREE_WIDTH];
                        new_values[key.subindex()] = Some(value);
                        *left = Some(Box::new(Node::Stem {
                            stem: key.stem(),
                            values: Box::new(new_values),
                        }));
                    }
                } else if let Some(right) = right {
                    right.insert(key, value, depth + 1);
                } else {
                    let mut new_values = [None; STEM_SUBTREE_WIDTH];
                    new_values[key.subindex()] = Some(value);
                    if let Node::Internal { right, .. } = self {
                        *right = Some(Box::new(Node::Stem {
                            stem: key.stem(),
                            values: Box::new(new_values),
                        }));
                    }
                }
            }
        }
    }

    fn get_bit(stem: &[u8], depth: u8) -> u8 {
        stem[depth as usize / 8] & (1 << (7 - depth % 8))
    }
}
