use router::trie::Trie;

#[path = "../fixtures/oci.rs"]
mod oci;

#[test]
fn test() {
    let mut trie = Trie::new();
    let mut counter = 0;
    for route in oci::ROUTES {
        counter += 1;
        trie.insert(route, counter);
    }

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ /v2 [1]
            ╰─ /
               ╰─ {name:*}
                         ╰─ /
                            ├─ blobs/
                            │       ├─ uploads [3]
                            │       │        ╰─ /
                            │       │           ╰─ {reference} [4]
                            │       ╰─ {digest} [2]
                            ├─ manifests/
                            │           ╰─ {reference} [5]
                            ├─ referrers/
                            │           ╰─ {digest} [6]
                            ╰─ tags/list [7]
    "###);
}
