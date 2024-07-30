//! Tests stolen from `poem` (MIT OR Apache-2.0)
//! <https://github.com/poem-web/poem/blob/0b6ca89be9636472b25f3677dc957fe098f72fab/poem/src/route/internal/radix_tree.rs>

use router::{assert_router_matches, trie::Trie};

#[test]
fn test_insert_static_child_1() {
    let mut trie = Trie::new();
    trie.insert("/abc", 1);
    trie.insert("/abcdef", 2);
    trie.insert("/abcdefgh", 3);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /abc [1]
             ╰─ def [2]
                  ╰─ gh [3]
    "###);
}

#[test]
fn test_insert_static_child_2() {
    let mut trie = Trie::new();
    trie.insert("/abcd", 1);
    trie.insert("/ab1234", 2);
    trie.insert("/ab1256", 3);
    trie.insert("/ab125678", 4);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /ab
            ├─ cd [1]
            ╰─ 12
                ├─ 34 [2]
                ╰─ 56 [3]
                    ╰─ 78 [4]
    "###);
}

#[test]
fn test_insert_static_child_3() {
    let mut trie = Trie::new();
    trie.insert("/abc", 1);
    trie.insert("/ab", 2);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /ab [2]
            ╰─ c [1]
    "###);
}

#[test]
fn test_insert_dynamic_children() {
    let mut trie = Trie::new();
    trie.insert("/abc/{p1}", 1);
    trie.insert("/abc/{p1}/p2", 2);
    trie.insert("/abc/{p1}/{p3}", 3);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /abc/
              ╰─ {p1} [1]
                    ╰─ /
                       ├─ p2 [2]
                       ╰─ {p3} [3]
    "###);
}

#[test]
fn test_catch_all_child_1() {
    let mut trie = Trie::new();
    trie.insert("/abc/{p1:*}", 1);
    trie.insert("/ab/de", 2);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /ab
            ├─ c/
            │   ╰─ {p1:*} [1]
            ╰─ /de [2]
    "###);
}

#[test]
fn test_catch_all_child_2() {
    let mut trie = Trie::new();
    trie.insert("{p1:*}", 1);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ {p1:*} [1]
    "###);
}

#[test]
#[cfg(feature = "regex")]
#[ignore]
fn test_insert_regex_child() {
    todo!();
}

#[test]
#[ignore]
fn test_insert_result() {
    todo!();
}

#[test]
#[cfg(feature = "regex")]
fn test_matches() {
    let mut trie = Trie::new();
    trie.insert("/ab/def", 1);
    trie.insert("/abc/def", 2);
    trie.insert("/abc/{p1}", 3);
    trie.insert("/abc/{p1}/def", 4);
    trie.insert("/abc/{p1}/{p2}", 5);
    trie.insert("/abc/def/{p1:*}", 6);
    trie.insert("/a/b/c/d", 7);
    trie.insert("/a/{p1}/{p2}/c", 8);
    trie.insert("/{p1:*}", 9);
    trie.insert("/abc/{p1:\\d+}/def", 10);
    trie.insert("/kcd/{p1:\\d+}", 11);
    trie.insert("/{package}/-/{package_tgz:.*tgz$}", 12);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /
          ├─ a
          │  ├─ b
          │  │  ├─ /def [1]
          │  │  ╰─ c/
          │  │      ├─ def [2]
          │  │      │    ╰─ /
          │  │      │       ╰─ {p1:*} [6]
          │  │      ├─ {p1:\d+}
          │  │      │         ╰─ /def [10]
          │  │      ╰─ {p1} [3]
          │  │            ╰─ /
          │  │               ├─ def [4]
          │  │               ╰─ {p2} [5]
          │  ╰─ /
          │     ├─ b/c/d [7]
          │     ╰─ {p1}
          │           ╰─ /
          │              ╰─ {p2}
          │                    ╰─ /c [8]
          ├─ kcd/
          │     ╰─ {p1:\d+} [11]
          ├─ {package}
          │          ╰─ /-/
          │               ╰─ {package_tgz:.*tgz$} [12]
          ╰─ {p1:*} [9]
    "###);

    assert_router_matches!(trie, {
        "/ab/def" => {
            path: "/ab/def",
            value: 1
        }
        "/abc/def" => {
            path: "/abc/def",
            value: 2
        }
        "/abc/cde" => {
            path: "/abc/{p1}",
            value: 3,
            params: {
                "p1" => "cde"
            }
        }
        "/abc/cde/def" => {
            path: "/abc/{p1}/def",
            value: 4,
            params: {
                "p1" => "cde"
            }
        }
        "/abc/cde/hjk" => {
            path: "/abc/{p1}/{p2}",
            value: 5,
            params: {
                "p1" => "cde",
                "p2" => "hjk"
            }
        }
        "/abc/def/iop/123" => {
            path: "/abc/def/{p1:*}",
            value: 6
        }
        "/a/b/k/c" => {
            path: "/a/{p1}/{p2}/c",
            value: 8,
            params: {
                "p1" => "",
                "p2" => "k"
            }
        }
        "/kcd/uio" => {
            path: "/{p1:*}",
            value: 9,
            params: {
                "p1" => "kcd/uio"
            }
        }
        "/" => None
        "/abc/123/def" => {
            path: "/abc/{p1:\\d+}/def",
            value: 10,
            params: {
                "p1" => "123"
            }
        }
        "/kcd/567" => {
            path: "/kcd/{p1:\\d+}",
            value: 11,
            params: {
                "p1" => "567"
            }
        }
        "/is-number/-/is-number-7.0.0.tgz" => {
            path: "/{package}/-/{package_tgz:.*tgz$}",
            value: 12,
            params: {
                "package" => "is-number",
                "package_tgz" => "is-number-7.0.0.tgz"
            }
        }
    });
}

#[test]
fn test_match_priority() {
    let mut trie = Trie::new();
    trie.insert("/a/bc", 1);
    trie.insert("/a/{path:*}", 2);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /a/
            ├─ bc [1]
            ╰─ {path:*} [2]
    "###);

    assert_router_matches!(trie, {
        "/a/123" => {
            path: "/a/{path:*}",
            value: 2,
            params: {
                "path" => "123"
            }
        }
    });

    trie.insert("/a/{id}", 3);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /a/
            ├─ bc [1]
            ├─ {id} [3]
            ╰─ {path:*} [2]
    "###);

    assert_router_matches!(trie, {
        "/a/123" => {
            path: "/a/{id}",
            value: 3,
            params: {
                "id" => "123"
            }
        }
    });

    // todo!()
    // tree.add("/a/:id<\\d+>", 4).unwrap();
    // let matches = tree.matches("/a/123");
    // assert_eq!(matches.unwrap().data.data, 4);

    trie.insert("/a/123", 5);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /a/
            ├─ bc [1]
            ├─ 123 [5]
            ├─ {id} [3]
            ╰─ {path:*} [2]
    "###);

    assert_router_matches!(trie, {
        "/a/123" => {
            path: "/a/123",
            value: 5
        }
    });
}

#[test]
fn test_catch_all_priority_in_sub_path() {
    let mut trie = Trie::new();
    trie.insert("/a/{path:*}", 1);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /a/
            ╰─ {path:*} [1]
    "###);

    assert_router_matches!(trie, {
        "/a/b/c/123" => {
            path: "/a/{path:*}",
            value: 1,
            params: {
                "path" => "b/c/123"
            }
        }
    });

    trie.insert("/a/b/{path:*}", 2);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /a/
            ├─ b/
            │   ╰─ {path:*} [2]
            ╰─ {path:*} [1]
    "###);

    assert_router_matches!(trie, {
        "/a/b/c/123" => {
            path: "/a/b/{path:*}",
            value: 2,
            params: {
                "path" => "c/123"
            }
        }
    });

    trie.insert("/a/b/c/{path:*}", 3);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /a/
            ├─ b/
            │   ├─ c/
            │   │   ╰─ {path:*} [3]
            │   ╰─ {path:*} [2]
            ╰─ {path:*} [1]
    "###);

    assert_router_matches!(trie, {
        "/a/b/c/123" => {
            path: "/a/b/c/{path:*}",
            value: 3,
            params: {
                "path" => "123"
            }
        }
    });
}

#[test]
fn test_issue_275() {
    let mut trie = Trie::new();
    trie.insert("/{id1}/a", 1);
    trie.insert("/{id2}/b", 2);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /
          ├─ {id1}
          │      ╰─ /a [1]
          ╰─ {id2}
                 ╰─ /b [2]
    "###);

    assert_router_matches!(trie, {
        "/abc/a" => {
            path: "/{id1}/a",
            value: 1,
            params: {
                "id1" => "abc"
            }
        }
        "/def/b" => {
            path: "/{id2}/b",
            value: 2,
            params: {
                "id2" => "def"
            }
        }
    });
}

#[test]
fn test_percent_decoded() {
    let mut trie = Trie::new();
    trie.insert("/a/{id}", 1);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /a/
            ╰─ {id} [1]
    "###);

    assert_router_matches!(trie, {
        "/a/abc" => {
            path: "/a/{id}",
            value: 1,
            params: {
                "id" => "abc"
            }
        }
        "/a/%E4%BD%A0%E5%A5%BD" => {
            path: "/a/{id}",
            value: 1,
            params: {
                "id" => "%E4%BD%A0%E5%A5%BD"
            }
        }
    });
}
