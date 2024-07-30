use divan::AllocProfiler;

#[path = "../fixtures/simple.rs"]
mod simple;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

mod matches {
    #[divan::bench]
    fn mine_matches(bencher: divan::Bencher) {
        let mut mine = router::trie::Trie::new();
        for route in crate::simple::ROUTES {
            mine.insert(route, 0);
        }

        bencher.bench_local(|| {
            for search in crate::simple::SEARCHES {
                criterion::black_box(mine.matches(search).unwrap());
            }
        });
    }

    #[divan::bench]
    fn matchit_matches(bencher: divan::Bencher) {
        let mut matchit = matchit::Router::new();
        for route in crate::simple::ROUTES {
            matchit.insert(route, 0).unwrap();
        }

        bencher.bench_local(|| {
            for search in crate::simple::SEARCHES {
                criterion::black_box(matchit.at(search).unwrap());
            }
        });
    }

    #[divan::bench]
    fn gonzales_matches(bencher: divan::Bencher) {
        let gonzales = gonzales::RouterBuilder::new().build(crate::simple::ROUTES);

        bencher.bench_local(|| {
            for search in crate::simple::SEARCHES {
                criterion::black_box(gonzales.route(search).unwrap());
            }
        });
    }
}

// mod e2e {
//     #[divan::bench]
//     fn mine_e2e() {
//         let mut mine = router::trie::Trie::new();
//
//         for route in crate::simple::ROUTES {
//             mine.insert(route, 0);
//         }
//
//         for search in crate::simple::SEARCHES {
//             criterion::black_box(mine.matches(search).unwrap());
//         }
//     }
//
//     #[divan::bench]
//     fn matchit_e2e() {
//         let mut matchit = matchit::Router::new();
//         for route in crate::simple::ROUTES {
//             matchit.insert(route, 0).unwrap();
//         }
//
//         for search in crate::simple::SEARCHES {
//             criterion::black_box(matchit.at(search).unwrap());
//         }
//     }
//
//     #[divan::bench]
//     fn gonzales_e2e() {
//         let gonzales = gonzales::RouterBuilder::new().build(crate::simple::ROUTES);
//
//         for search in crate::simple::SEARCHES {
//             criterion::black_box(gonzales.route(search).unwrap());
//         }
//     }
// }
