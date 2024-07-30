use crate::{matches::Parameter, segment::Segment, segment::Segments};
use smallvec::{smallvec, SmallVec};
use std::fmt::Display;

#[cfg(feature = "regex")]
use regex::bytes::Regex;

pub enum NodeKind {
    Root,
    Static,
    #[cfg(feature = "regex")]
    Regex(Regex),
    Dynamic,
    Wildcard,
    EndWildcard,
}

#[derive(Debug, Eq, PartialEq)]
pub struct NodeData<'a, T> {
    pub path: &'a str,
    pub value: T,
}

pub struct Node<'a, T> {
    pub kind: NodeKind,

    pub prefix: &'a [u8],
    pub data: Option<NodeData<'a, T>>,

    pub static_children: Vec<Node<'a, T>>,
    #[cfg(feature = "regex")]
    pub regex_children: Vec<Node<'a, T>>,
    pub dynamic_children: Vec<Node<'a, T>>,
    pub wildcard_children: Vec<Node<'a, T>>,
    pub end_wildcard: Option<Box<Node<'a, T>>>,

    // TODO: Better name needed.
    pub quick_dynamic: bool,
}

impl<'a, T: Display> Node<'a, T> {
    pub fn insert(&mut self, mut segments: Segments<'a>, data: NodeData<'a, T>) {
        if let Some(segment) = segments.0.pop() {
            match segment {
                Segment::Static { prefix } => self.insert_static(segments, data, prefix),
                #[cfg(feature = "regex")]
                Segment::Regex { prefix, regex } => self.insert_regex(segments, data, prefix, regex),
                Segment::Dynamic { prefix } => self.insert_dynamic(segments, data, prefix),
                Segment::Wildcard { prefix } if segments.0.is_empty() => self.insert_end_wildcard(data, prefix),
                Segment::Wildcard { prefix } => self.insert_wildcard(segments, data, prefix),
            }
        } else {
            assert!(self.data.is_none(), "Duplicate path");
            self.data = Some(data);
        }

        self.update_quick_dynamic();
    }

    fn insert_static(&mut self, segments: Segments<'a>, data: NodeData<'a, T>, prefix: &'a [u8]) {
        let Some(child) = self
            .static_children
            .iter_mut()
            .find(|child| child.prefix[0] == prefix[0])
        else {
            self.static_children.push({
                let mut new_child = Self {
                    kind: NodeKind::Static,
                    prefix,
                    ..Self::default()
                };

                new_child.insert(segments, data);
                new_child
            });

            return;
        };

        let common_prefix = prefix
            .iter()
            .zip(child.prefix)
            .take_while(|&(x, y)| x == y)
            .count();

        if common_prefix >= child.prefix.len() {
            if common_prefix >= prefix.len() {
                child.insert(segments, data);
            } else {
                child.insert_static(segments, data, &prefix[common_prefix..]);
            }

            return;
        }

        let new_child_a = Self {
            kind: NodeKind::Static,

            prefix: &child.prefix[common_prefix..],
            data: child.data.take(),

            static_children: std::mem::take(&mut child.static_children),
            #[cfg(feature = "regex")]
            regex_children: std::mem::take(&mut child.regex_children),
            dynamic_children: std::mem::take(&mut child.dynamic_children),
            wildcard_children: std::mem::take(&mut child.wildcard_children),
            end_wildcard: std::mem::take(&mut child.end_wildcard),

            quick_dynamic: child.quick_dynamic,
        };

        let new_child_b = Self {
            kind: NodeKind::Static,
            prefix: &prefix[common_prefix..],
            ..Self::default()
        };

        child.prefix = &child.prefix[..common_prefix];

        if prefix[common_prefix..].is_empty() {
            child.static_children = vec![new_child_a];
            child.insert(segments, data);
        } else {
            child.static_children = vec![new_child_a, new_child_b];
            child.static_children[1].insert(segments, data);
        }
    }

    #[cfg(feature = "regex")]
    fn insert_regex(&mut self, segments: Segments<'a>, data: NodeData<'a, T>, prefix: &'a [u8], regex: Regex) {
        if let Some(child) = self
            .regex_children
            .iter_mut()
            .find(|child| child.prefix == prefix)
        {
            child.insert(segments, data);
        } else {
            self.regex_children.push({
                let mut new_child = Self {
                    kind: NodeKind::Regex(regex),
                    prefix,
                    ..Self::default()
                };

                new_child.insert(segments, data);
                new_child
            });
        }
    }

    fn insert_dynamic(&mut self, segments: Segments<'a>, data: NodeData<'a, T>, prefix: &'a [u8]) {
        if let Some(child) = self
            .dynamic_children
            .iter_mut()
            .find(|child| child.prefix == prefix)
        {
            child.insert(segments, data);
        } else {
            self.dynamic_children.push({
                let mut new_child = Self {
                    kind: NodeKind::Dynamic,
                    prefix,
                    ..Self::default()
                };

                new_child.insert(segments, data);
                new_child
            });
        }
    }

    fn insert_wildcard(&mut self, segments: Segments<'a>, data: NodeData<'a, T>, prefix: &'a [u8]) {
        if let Some(child) = self
            .wildcard_children
            .iter_mut()
            .find(|child| child.prefix == prefix)
        {
            child.insert(segments, data);
        } else {
            self.wildcard_children.push({
                let mut new_child = Self {
                    kind: NodeKind::Wildcard,
                    prefix,
                    ..Self::default()
                };

                new_child.insert(segments, data);
                new_child
            });
        }
    }

    fn insert_end_wildcard(&mut self, data: NodeData<'a, T>, prefix: &'a [u8]) {
        // FIXME: We probably need splitting capabilities here, to change an end wildcard into a normal wildcard?
        self.end_wildcard = Some(Box::new(Self {
            kind: NodeKind::EndWildcard,
            prefix,
            data: Some(data),
            ..Self::default()
        }));
    }

    fn update_quick_dynamic(&mut self) {
        self.quick_dynamic = self
            .dynamic_children
            .iter()
            .all(|child| {
                // Leading slash?
                if child.prefix.first() == Some(&b'/') {
                    return true;
                }

                // Has no children?
                #[cfg(not(feature = "regex"))]
                let no_regex_children = true;
                #[cfg(feature = "regex")]
                let no_regex_children = child.regex_children.is_empty();

                if child.static_children.is_empty()
                    && child.dynamic_children.is_empty()
                    && child.wildcard_children.is_empty()
                    && child.end_wildcard.is_none()
                    && no_regex_children
                {
                    return true;
                }

                // All static children start with a slash?
                if child
                    .static_children
                    .iter()
                    .all(|child| child.prefix.first() == Some(&b'/'))
                {
                    return true;
                }

                false
            });

        for child in &mut self.static_children {
            child.update_quick_dynamic();
        }

        #[cfg(feature = "regex")]
        for child in &mut self.regex_children {
            child.update_quick_dynamic();
        }

        for child in &mut self.dynamic_children {
            child.update_quick_dynamic();
        }

        for child in &mut self.wildcard_children {
            child.update_quick_dynamic();
        }

        if let Some(child) = self.end_wildcard.as_mut() {
            child.update_quick_dynamic();
        }
    }

    pub fn matches(
        &'a self,
        path: &'a [u8],
        parameters: &mut SmallVec<[Parameter<'a>; 4]>,
    ) -> Option<&'a NodeData<'a, T>> {
        if path.is_empty() {
            return self.data.as_ref();
        }

        if let Some(matches) = self.matches_static(path, parameters) {
            return Some(matches);
        }

        #[cfg(feature = "regex")]
        if let Some(matches) = self.matches_regex(path, parameters) {
            return Some(matches);
        }

        if let Some(matches) = self.matches_dynamic(path, parameters) {
            return Some(matches);
        }

        if let Some(matches) = self.matches_wildcard(path, parameters) {
            return Some(matches);
        }

        if let Some(matches) = self.matches_end_wildcard(path, parameters) {
            return Some(matches);
        }

        None
    }

    fn matches_static(
        &'a self,
        path: &'a [u8],
        parameters: &mut SmallVec<[Parameter<'a>; 4]>,
    ) -> Option<&'a NodeData<'a, T>> {
        for static_child in &self.static_children {
            // NOTE: This was previously a "starts_with" call, but turns out this is much faster.
            if path.len() >= static_child.prefix.len()
                && static_child
                    .prefix
                    .iter()
                    .zip(path)
                    .all(|(a, b)| a == b)
            {
                let remaining_path = &path[static_child.prefix.len()..];
                if let Some(node_data) = static_child.matches(remaining_path, parameters) {
                    return Some(node_data);
                }
            }
        }

        None
    }

    #[cfg(feature = "regex")]
    fn matches_regex(
        &'a self,
        path: &'a [u8],
        parameters: &mut SmallVec<[Parameter<'a>; 4]>,
    ) -> Option<&'a NodeData<'a, T>> {
        for regex_child in &self.regex_children {
            let NodeKind::Regex(ref regex) = regex_child.kind else {
                continue;
            };

            let segment_end = path
                .iter()
                .position(|&b| b == b'/')
                .unwrap_or(path.len());

            let segment = &path[..segment_end];

            if let Some(captures) = regex.captures(segment) {
                if let Some(matched) = captures.get(0) {
                    parameters.push(Parameter {
                        key: regex_child.prefix,
                        value: matched.as_bytes(),
                    });

                    if let Some(node_data) = regex_child.matches(&path[segment_end..], parameters) {
                        return Some(node_data);
                    }

                    parameters.pop();
                }
            }
        }

        None
    }

    fn matches_dynamic(
        &'a self,
        path: &'a [u8],
        parameters: &mut SmallVec<[Parameter<'a>; 4]>,
    ) -> Option<&'a NodeData<'a, T>> {
        if self.quick_dynamic {
            self.matches_dynamic_segment(path, parameters)
        } else {
            self.matches_dynamic_inline(path, parameters)
        }
    }

    // Dynamic with support for inline dynamic sections, e.g. `{name}.{extension}`
    // NOTE: Parameters are greedy in nature:
    //   Route: `{name}.{extension}`
    //   Path: `my.long.file.txt`
    //   Name: `my.long.file`
    //   Ext: `txt`
    fn matches_dynamic_inline(
        &'a self,
        path: &'a [u8],
        parameters: &mut SmallVec<[Parameter<'a>; 4]>,
    ) -> Option<&'a NodeData<'a, T>> {
        for dynamic_child in &self.dynamic_children {
            let mut consumed = 0;

            // FIXME: Reliant on the ordering, which is currently dependant on insert order.
            let mut last_match = None;
            let mut last_match_parameters = smallvec![];

            while consumed < path.len() {
                if path[consumed] == b'/' {
                    break;
                }

                consumed += 1;

                let mut current_parameters = parameters.clone();
                current_parameters.push(Parameter {
                    key: dynamic_child.prefix,
                    value: &path[..consumed],
                });

                if let Some(node_data) = dynamic_child.matches(&path[consumed..], &mut current_parameters) {
                    last_match = Some(node_data);
                    last_match_parameters = current_parameters;
                }
            }

            if let Some(node_data) = last_match {
                *parameters = last_match_parameters;
                return Some(node_data);
            }
        }

        None
    }

    // Doesn't support inline dynamic sections, e.g. `{name}.{extension}`, only `/{segment}/`
    fn matches_dynamic_segment(
        &'a self,
        path: &'a [u8],
        parameters: &mut SmallVec<[Parameter<'a>; 4]>,
    ) -> Option<&'a NodeData<'a, T>> {
        for dynamic_child in &self.dynamic_children {
            let segment_end = path
                .iter()
                .position(|&b| b == b'/')
                .unwrap_or(path.len());

            parameters.push(Parameter {
                key: dynamic_child.prefix,
                value: &path[..segment_end],
            });

            if let Some(node_data) = dynamic_child.matches(&path[segment_end..], parameters) {
                return Some(node_data);
            }

            parameters.pop();
        }

        None
    }

    fn matches_wildcard(
        &'a self,
        path: &'a [u8],
        parameters: &mut SmallVec<[Parameter<'a>; 4]>,
    ) -> Option<&'a NodeData<'a, T>> {
        for wildcard_child in &self.wildcard_children {
            let mut consumed = 0;
            let mut remaining_path = path;
            let mut section_end = false;

            while !remaining_path.is_empty() {
                if section_end {
                    consumed += 1;
                }

                let segment_end = remaining_path
                    .iter()
                    .position(|&b| b == b'/')
                    .unwrap_or(remaining_path.len());

                if segment_end == 0 {
                    consumed += 1;
                    section_end = false;
                } else {
                    consumed += segment_end;
                    section_end = true;
                }

                parameters.push(Parameter {
                    key: wildcard_child.prefix,
                    value: if path[..consumed].ends_with(b"/") {
                        &path[..consumed - 1]
                    } else {
                        &path[..consumed]
                    },
                });

                if let Some(node_data) = wildcard_child.matches(&remaining_path[segment_end..], parameters) {
                    return Some(node_data);
                }

                parameters.pop();

                if segment_end == remaining_path.len() {
                    break;
                }

                remaining_path = &remaining_path[segment_end + 1..];
            }
        }

        None
    }

    fn matches_end_wildcard(
        &'a self,
        path: &'a [u8],
        parameters: &mut SmallVec<[Parameter<'a>; 4]>,
    ) -> Option<&'a NodeData<'a, T>> {
        if let Some(end_wildcard) = &self.end_wildcard {
            parameters.push(Parameter {
                key: end_wildcard.prefix,
                value: path,
            });

            return end_wildcard.data.as_ref();
        }

        None
    }
}

impl<'a, T: Display> Default for Node<'a, T> {
    fn default() -> Self {
        Self {
            kind: NodeKind::Root,

            prefix: b"",
            data: None,

            static_children: vec![],
            #[cfg(feature = "regex")]
            regex_children: vec![],
            dynamic_children: vec![],
            wildcard_children: vec![],
            end_wildcard: None,

            quick_dynamic: true,
        }
    }
}
