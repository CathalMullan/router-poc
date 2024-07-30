use crate::{
    node::{Node, NodeKind},
    trie::Trie,
};
use std::fmt::Display;

impl<'a, T: Display> Display for Node<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn debug_node<T: Display>(
            f: &mut std::fmt::Formatter,
            node: &Node<T>,
            padding: &str,
            is_root: bool,
            is_last: bool,
        ) -> std::fmt::Result {
            let key = match node.kind {
                NodeKind::Root => "ROOT",
                NodeKind::Static => &String::from_utf8_lossy(node.prefix),
                #[cfg(feature = "regex")]
                NodeKind::Regex(ref regex) => {
                    let prefix = String::from_utf8_lossy(node.prefix);
                    &format!("{{{prefix}:{regex}}}")
                }
                NodeKind::Dynamic => {
                    let prefix = String::from_utf8_lossy(node.prefix);
                    &format!("{{{prefix}}}")
                }
                NodeKind::Wildcard | NodeKind::EndWildcard => {
                    let prefix = String::from_utf8_lossy(node.prefix);
                    &format!("{{{prefix}:*}}")
                }
            };

            let value = node
                .data
                .as_ref()
                .map(|node_data| &node_data.value);

            if is_root {
                writeln!(f, "{key}")?;
            } else if is_last {
                match value {
                    Some(value) => writeln!(f, "{padding}╰─ {key} [{value}]")?,
                    None => writeln!(f, "{padding}╰─ {key}")?,
                }
            } else {
                match value {
                    Some(value) => writeln!(f, "{padding}├─ {key} [{value}]")?,
                    None => writeln!(f, "{padding}├─ {key}")?,
                }
            }

            let extra_spacing = " ".repeat(key.len() - 1);
            let new_prefix = if is_root {
                padding.to_string()
            } else if is_last {
                format!("{padding}   {extra_spacing}")
            } else {
                format!("{padding}│  {extra_spacing}")
            };

            #[cfg(not(feature = "regex"))]
            let has_regex_children = false;
            #[cfg(feature = "regex")]
            let has_regex_children = !node.regex_children.is_empty();

            let has_dynamic_children = !node.dynamic_children.is_empty();
            let has_wildcard_children = !node.wildcard_children.is_empty();
            let has_end_wildcard = node.end_wildcard.is_some();

            // Recursively print the static children
            let static_count = node.static_children.len();
            for (index, child) in node.static_children.iter().enumerate() {
                let is_last = if has_regex_children || has_dynamic_children || has_wildcard_children || has_end_wildcard
                {
                    false
                } else {
                    index == (static_count - 1)
                };

                debug_node(f, child, &new_prefix, false, is_last)?;
            }

            // Recursively print regex children
            #[cfg(feature = "regex")]
            {
                let regex_count = node.regex_children.len();
                for (index, child) in node.regex_children.iter().enumerate() {
                    let is_last = if has_dynamic_children || has_wildcard_children || has_end_wildcard {
                        false
                    } else {
                        index == (regex_count - 1)
                    };

                    debug_node(f, child, &new_prefix, false, is_last)?;
                }
            }

            // Recursively print dynamic children
            let dynamic_count = node.dynamic_children.len();
            for (index, child) in node.dynamic_children.iter().enumerate() {
                let is_last = if has_wildcard_children || has_end_wildcard {
                    false
                } else {
                    index == (dynamic_count - 1)
                };

                debug_node(f, child, &new_prefix, false, is_last)?;
            }

            // Recursively print wildcard children
            let wildcard_count = node.wildcard_children.len();
            for (index, child) in node
                .wildcard_children
                .iter()
                .enumerate()
            {
                let is_last = if has_end_wildcard {
                    false
                } else {
                    index == (wildcard_count - 1)
                };

                debug_node(f, child, &new_prefix, false, is_last)?;
            }

            // Print end wildcard
            if let Some(child) = &node.end_wildcard {
                debug_node(f, child, &new_prefix, false, true)?;
            }

            Ok(())
        }

        let padding = if self.prefix.is_empty() {
            "   ".to_string()
        } else {
            " ".repeat(self.prefix.len() - 1)
        };

        debug_node(f, self, &padding, true, true)?;
        Ok(())
    }
}

impl<'a, T: Display> Display for Trie<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}

#[cfg(test)]
mod tests {
    use crate::trie::Trie;

    #[test]
    fn test_node_debug_nested() {
        let mut trie = Trie::new();
        trie.insert("/abcd", 1);
        trie.insert("/abe", 2);
        trie.insert("/fg", 3);

        insta::assert_snapshot!(trie, @r###"
        ROOT
           ╰─ /
              ├─ ab
              │   ├─ cd [1]
              │   ╰─ e [2]
              ╰─ fg [3]
        "###);
    }

    #[test]
    fn test_node_debug_flat() {
        let mut trie = Trie::new();
        trie.insert("/about-us/team", 1);
        trie.insert("/blog/{post}", 2);
        trie.insert("/contact", 3);
        trie.insert("/downloads/{path:*}/{file}.{extension}", 4);
        trie.insert("/search", 5);
        trie.insert("/support", 6);
        trie.insert("/{catch_all:*}", 7);

        insta::assert_snapshot!(trie, @r###"
        ROOT
           ╰─ /
              ├─ about-us/team [1]
              ├─ blog/
              │      ╰─ {post} [2]
              ├─ contact [3]
              ├─ downloads/
              │           ╰─ {path:*}
              │                     ╰─ /
              │                        ╰─ {file}
              │                                ╰─ .
              │                                   ╰─ {extension} [4]
              ├─ s
              │  ├─ earch [5]
              │  ╰─ upport [6]
              ╰─ {catch_all:*} [7]
        "###);
    }
}
