use router::{assert_router_matches, trie::Trie};

#[test]
fn test_multibyte_characters() {
    let mut trie = Trie::new();
    trie.insert("/☃/{id}/🌈", 1);

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /☃/
              ╰─ {id}
                    ╰─ /🌈 [1]
    "###);

    assert_router_matches!(trie, {
        "/☃/abc/🌈" => {
            path: "/☃/{id}/🌈",
            value: 1,
            params: {
                "id" => "abc"
            }
        }
        "/☃/é/🌈" => {
            path: "/☃/{id}/🌈",
            value: 1,
            params: {
                "id" => "é"
            }
        }
        "/☃abc/🌈" => None
    });
}
