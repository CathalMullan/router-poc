use crate::{
    matches::Match,
    node::{Node, NodeData},
    segment::Segments,
};
use smallvec::smallvec;
use std::fmt::Display;

pub struct Trie<'a, T> {
    pub root: Node<'a, T>,
}

impl<'a, T: Display> Trie<'a, T> {
    #[must_use]
    pub fn new() -> Self {
        Self { root: Node::default() }
    }

    pub fn insert(&mut self, path: &'a str, value: T) {
        let path_bytes = path.as_bytes();

        self.root
            .insert(Segments::new(path_bytes), NodeData { path, value });
    }

    #[must_use]
    pub fn matches(&'a self, path: &'a str) -> Option<Match<'a, T>> {
        let path = path.as_bytes();

        let mut parameters = smallvec![];
        let data = self
            .root
            .matches(path, &mut parameters)?;

        Some(Match { data, parameters })
    }
}

impl<'a, T: Display> Default for Trie<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}
