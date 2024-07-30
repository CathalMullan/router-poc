use std::fmt::{Debug, Display};

#[cfg(feature = "regex")]
use regex::bytes::Regex;

#[derive(Debug)]
pub struct Segments<'a>(pub Vec<Segment<'a>>);

impl<'a> Segments<'a> {
    #[must_use]
    pub fn new(path: &'a [u8]) -> Self {
        let mut index = 0;
        let mut segments = vec![];
        let mut start = 0;

        while index < path.len() {
            if path[index] == b'{' {
                if index > start {
                    segments.push(Segment::Static {
                        prefix: &path[start..index],
                    });
                }

                let (name, pattern, new_index) = Self::parse_name(path, index + 1);
                match pattern {
                    Some(p) if p.starts_with(b"*") => {
                        segments.push(Segment::Wildcard { prefix: name });
                    }
                    #[cfg(feature = "regex")]
                    Some(p) => {
                        segments.push(Segment::Regex {
                            prefix: name,
                            regex: Regex::new(std::str::from_utf8(p).unwrap()).unwrap(),
                        });
                    }
                    _ => {
                        segments.push(Segment::Dynamic { prefix: name });
                    }
                }

                index = new_index;
                start = index;
            } else {
                let static_path = Self::parse_static(path, &mut index);
                if static_path.is_empty() {
                    index += 1;
                } else {
                    segments.push(Segment::Static { prefix: static_path });
                    start = index;
                }
            }
        }

        if start < path.len() {
            segments.push(Segment::Static { prefix: &path[start..] });
        }

        segments.reverse();
        Self(segments)
    }

    fn parse_name(path: &'a [u8], index: usize) -> (&'a [u8], Option<&'a [u8]>, usize) {
        let start = index;
        let mut colon_index = None;
        let mut end = index;

        while end < path.len() {
            match path[end] {
                b'}' => {
                    end += 1;
                    if let Some(ci) = colon_index {
                        let name = &path[start..ci];
                        let pattern = &path[ci + 1..end - 1];
                        return (name, Some(pattern), end);
                    }

                    let name = &path[start..end - 1];
                    return (name, None, end);
                }
                b':' if colon_index.is_none() => {
                    colon_index = Some(end);
                    end += 1;
                }
                _ => end += 1,
            }
        }

        (&path[start..end], None, end)
    }

    fn parse_static(path: &'a [u8], index: &mut usize) -> &'a [u8] {
        let start = *index;

        while *index < path.len() {
            if path[*index] == b'{' {
                break;
            }

            *index += 1;
        }

        &path[start..*index]
    }
}

impl<'a> Display for Segments<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut segments: Vec<String> = self
            .0
            .iter()
            .map(|segment| match segment {
                Segment::Static { prefix } => String::from_utf8_lossy(prefix).into_owned(),
                #[cfg(feature = "regex")]
                Segment::Regex { prefix, regex } => {
                    format!("{{{}:{}}}", String::from_utf8_lossy(prefix), regex.as_str())
                }
                Segment::Dynamic { prefix } => format!("{{{}}}", String::from_utf8_lossy(prefix)),
                Segment::Wildcard { prefix } => format!("{{{}:*}}", String::from_utf8_lossy(prefix)),
            })
            .collect();

        segments.reverse();
        write!(f, "{}", segments.join(""))
    }
}

#[derive(Debug)]
pub enum Segment<'a> {
    Static {
        prefix: &'a [u8],
    },

    #[cfg(feature = "regex")]
    Regex {
        prefix: &'a [u8],
        regex: Regex,
    },

    Dynamic {
        prefix: &'a [u8],
    },

    Wildcard {
        prefix: &'a [u8],
    },
}

impl<'a> Display for Segment<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static { prefix } => write!(f, "Static(\"{}\")", String::from_utf8_lossy(prefix)),
            #[cfg(feature = "regex")]
            Self::Regex { prefix, regex } => write!(f, "Regex(\"{}\"): {regex}", String::from_utf8_lossy(prefix)),
            Self::Dynamic { prefix } => write!(f, "Dynamic(\"{}\")", String::from_utf8_lossy(prefix)),
            Self::Wildcard { prefix } => write!(f, "Wildcard(\"{}\")", String::from_utf8_lossy(prefix)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        // Test static path
        let segments = Segments::new(b"/abcd");
        assert_eq!(segments.0.len(), 1);
        assert!(matches!(segments.0[0], Segment::Static { prefix } if prefix == b"/abcd"));

        // Test path with regex segment
        #[cfg(feature = "regex")]
        {
            let segments = Segments::new(b"/{id:[0-9]+}");
            assert_eq!(segments.0.len(), 2);
            assert!(matches!(segments.0[1], Segment::Static { prefix } if prefix == b"/"));
            assert!(matches!(segments.0[0], Segment::Regex { prefix, .. } if prefix == b"id"));
        }

        // Test path with dynamic segment
        let segments = Segments::new(b"/{name}");
        assert_eq!(segments.0.len(), 2);
        assert!(matches!(segments.0[1], Segment::Static { prefix } if prefix == b"/"));
        assert!(matches!(segments.0[0], Segment::Dynamic { prefix } if prefix == b"name"));

        // Test path with wildcard segment
        let segments = Segments::new(b"/{path:*}");
        assert_eq!(segments.0.len(), 2);
        assert!(matches!(segments.0[1], Segment::Static { prefix } if prefix == b"/"));
        assert!(matches!(segments.0[0], Segment::Wildcard { prefix } if prefix == b"path"));
    }

    #[test]
    fn test_static_segment() {
        insta::assert_snapshot!(Segments::new(b"/hello"), @"/hello");
    }

    #[test]
    fn test_dynamic_segment() {
        insta::assert_snapshot!(Segments::new(b"/user/{id}"), @"/user/{id}");
    }

    #[test]
    #[cfg(feature = "regex")]
    fn test_regex_segment() {
        insta::assert_snapshot!(Segments::new(b"/user/{id:[0-9]+}"), @"/user/{id:[0-9]+}");
    }

    #[test]
    fn test_wildcard_segment() {
        insta::assert_snapshot!(Segments::new(b"/files/{path:*}"), @"/files/{path:*}");
    }

    #[test]
    fn test_mixed_segments() {
        insta::assert_snapshot!(Segments::new(b"/api/v1/{user_id:[0-9]+}/posts/{post_slug}/edit"), @"/api/v1/{user_id}/posts/{post_slug}/edit");
    }

    #[test]
    fn test_empty_path() {
        insta::assert_snapshot!(Segments::new(b""), @"");
    }

    #[test]
    fn test_root_path() {
        insta::assert_snapshot!(Segments::new(b"/"), @"/");
    }
}
