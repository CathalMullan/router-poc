use divan::AllocProfiler;

#[path = "../fixtures/advanced.rs"]
mod advanced;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

mod matches {
    #[divan::bench]
    fn mine_matches(bencher: divan::Bencher) {
        let mut mine = router::trie::Trie::new();

        for route in crate::advanced::ROUTES {
            mine.insert(route, 0);
        }

        bencher.bench_local(|| {
            for search in crate::advanced::SEARCHES {
                criterion::black_box(mine.matches(search).unwrap());
            }
        });
    }

    #[divan::bench]
    fn regex_matches(bencher: divan::Bencher) {
        let regex_set = regex::bytes::RegexSet::new(crate::advanced::ROUTES_REGEX).unwrap();

        bencher.bench_local(|| {
            for search in crate::advanced::SEARCHES {
                criterion::black_box(regex_set.matches(search.as_bytes()));
            }
        });
    }
}

// mod e2e {
//     #[divan::bench]
//     fn mine_e2e() {
//         let mut mine = router::trie::Trie::new();
//
//         for route in crate::advanced::ROUTES {
//             mine.insert(route, 0);
//         }
//
//         for search in crate::advanced::SEARCHES {
//             criterion::black_box(mine.matches(search).unwrap());
//         }
//     }
//
//     #[divan::bench]
//     fn regex_e2e() {
//         let regex_set = regex::bytes::RegexSet::new(crate::advanced::ROUTES_REGEX).unwrap();
//
//         for search in crate::advanced::SEARCHES {
//             criterion::black_box(regex_set.matches(search.as_bytes()));
//         }
//     }
// }
