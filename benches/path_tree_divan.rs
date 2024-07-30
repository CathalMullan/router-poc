use actix_router::{Path as ActixPath, Router as ActixRouter};
use divan::AllocProfiler;
use matchit::Router as MatchitRouter;
use ntex_router::{Path as NtexPath, Router as NtexRouter};
use path_table::PathTable;
use path_tree::PathTree;
use path_tree_routes::{ROUTES_URLS, ROUTES_WITH_BRACES, ROUTES_WITH_COLON};
use route_recognizer::Router as RRRouter;

#[path = "../fixtures/path_tree_routes.rs"]
mod path_tree_routes;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

mod matches {
    #[allow(clippy::wildcard_imports)]
    use crate::*;

    #[divan::bench]
    fn mine_matches(bencher: divan::Bencher) {
        let mut mine = router::trie::Trie::new();
        for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
            mine.insert(r, i);
        }

        bencher.bench_local(|| {
            for (i, r) in ROUTES_URLS.iter().enumerate() {
                let n = mine.matches(r).unwrap();
                assert_eq!(n.data.value, i);
            }
        });
    }

    #[divan::bench]
    fn actix_router_matches(bencher: divan::Bencher) {
        let mut router = ActixRouter::<usize>::build();
        for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
            router.path(*r, i);
        }
        let router = router.finish();

        bencher.bench_local(|| {
            for (i, r) in ROUTES_URLS.iter().enumerate() {
                let mut path = ActixPath::new(*r);
                let n = router.recognize(&mut path).unwrap();
                assert_eq!(*n.0, i);
            }
        });
    }

    #[divan::bench]
    fn ntex_router_matches(bencher: divan::Bencher) {
        let mut router = NtexRouter::<usize>::build();
        for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
            router.path(*r, i);
        }
        let router = router.finish();

        bencher.bench_local(|| {
            for (i, r) in ROUTES_URLS.iter().enumerate() {
                let mut path = NtexPath::new(*r);
                let n = router.recognize(&mut path).unwrap();
                assert_eq!(*n.0, i);
            }
        });
    }

    #[divan::bench]
    fn path_table_matches(bencher: divan::Bencher) {
        let mut table: PathTable<usize> = PathTable::new();
        for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
            *table.setup(r) = i;
        }

        bencher.bench_local(|| {
            for (i, r) in ROUTES_URLS.iter().enumerate() {
                let n = table.route(r).unwrap();
                assert_eq!(*n.0, i);
            }
        });
    }

    #[divan::bench]
    fn path_tree_matches(bencher: divan::Bencher) {
        let mut tree: PathTree<usize> = PathTree::new();
        for (i, r) in ROUTES_WITH_COLON.iter().enumerate() {
            let _ = tree.insert(r, i);
        }

        bencher.bench_local(|| {
            for (i, r) in ROUTES_URLS.iter().enumerate() {
                let n = tree.find(r).unwrap();
                assert_eq!(*n.0, i);
            }
        });
    }

    #[divan::bench]
    fn matchit_matches(bencher: divan::Bencher) {
        let mut matcher = MatchitRouter::new();
        for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
            let _ = matcher.insert(*r, i);
        }

        bencher.bench_local(|| {
            for (i, r) in ROUTES_URLS.iter().enumerate() {
                let n = matcher.at(r).unwrap();
                assert_eq!(*n.value, i);
            }
        });
    }

    #[divan::bench]
    fn route_recognizer_matches(bencher: divan::Bencher) {
        let mut router = RRRouter::<usize>::new();
        for (i, r) in ROUTES_WITH_COLON.iter().enumerate() {
            router.add(r, i);
        }

        bencher.bench_local(|| {
            for (i, r) in ROUTES_URLS.iter().enumerate() {
                let n = router.recognize(r).unwrap();
                assert_eq!(**n.handler(), i);
            }
        });
    }
}

// mod e2e {
//     #[allow(clippy::wildcard_imports)]
//     use crate::*;
//
//     #[divan::bench]
//     fn mine_matches() {
//         let mut mine = router::trie::Trie::new();
//
//         for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
//             mine.insert(r, i);
//         }
//
//         for (i, r) in ROUTES_URLS.iter().enumerate() {
//             let n = mine.matches(r).unwrap();
//             assert_eq!(n.data.value, i);
//         }
//     }
//
//     #[divan::bench]
//     fn actix_router_matches() {
//         let mut router = ActixRouter::<usize>::build();
//         for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
//             router.path(*r, i);
//         }
//         let router = router.finish();
//
//         for (i, r) in ROUTES_URLS.iter().enumerate() {
//             let mut path = ActixPath::new(*r);
//             let n = router.recognize(&mut path).unwrap();
//             assert_eq!(*n.0, i);
//         }
//     }
//
//     #[divan::bench]
//     fn ntex_router_matches() {
//         let mut router = NtexRouter::<usize>::build();
//         for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
//             router.path(*r, i);
//         }
//         let router = router.finish();
//
//         for (i, r) in ROUTES_URLS.iter().enumerate() {
//             let mut path = NtexPath::new(*r);
//             let n = router.recognize(&mut path).unwrap();
//             assert_eq!(*n.0, i);
//         }
//     }
//
//     #[divan::bench]
//     fn path_table_matches() {
//         let mut table: PathTable<usize> = PathTable::new();
//         for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
//             *table.setup(r) = i;
//         }
//
//         for (i, r) in ROUTES_URLS.iter().enumerate() {
//             let n = table.route(r).unwrap();
//             assert_eq!(*n.0, i);
//         }
//     }
//
//     #[divan::bench]
//     fn path_tree_matches() {
//         let mut tree: PathTree<usize> = PathTree::new();
//         for (i, r) in ROUTES_WITH_COLON.iter().enumerate() {
//             let _ = tree.insert(r, i);
//         }
//
//         for (i, r) in ROUTES_URLS.iter().enumerate() {
//             let n = tree.find(r).unwrap();
//             assert_eq!(*n.0, i);
//         }
//     }
//
//     #[divan::bench]
//     fn matchit_matches() {
//         let mut matcher = MatchitRouter::new();
//         for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
//             let _ = matcher.insert(*r, i);
//         }
//
//         for (i, r) in ROUTES_URLS.iter().enumerate() {
//             let n = matcher.at(r).unwrap();
//             assert_eq!(*n.value, i);
//         }
//     }
//
//     #[divan::bench]
//     fn route_recognizer_matches() {
//         let mut router = RRRouter::<usize>::new();
//         for (i, r) in ROUTES_WITH_COLON.iter().enumerate() {
//             router.add(r, i);
//         }
//
//         for (i, r) in ROUTES_URLS.iter().enumerate() {
//             let n = router.recognize(r).unwrap();
//             assert_eq!(**n.handler(), i);
//         }
//     }
// }
