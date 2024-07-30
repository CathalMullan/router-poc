use actix_router::{Path as ActixPath, Router as ActixRouter};
use criterion::{criterion_group, criterion_main, Criterion};
use matchit::Router as MatchitRouter;
use ntex_router::{Path as NtexPath, Router as NtexRouter};
use path_table::PathTable;
use path_tree::PathTree;
use path_tree_routes::{ROUTES_URLS, ROUTES_WITH_BRACES, ROUTES_WITH_COLON};
use route_recognizer::Router as RRRouter;

#[path = "../fixtures/path_tree_routes.rs"]
mod path_tree_routes;

fn bench_path_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("routers");

    group
        .bench_function("mine", |b| {
            let mut mine = router::trie::Trie::new();
            for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
                mine.insert(r, i);
            }

            b.iter(|| {
                for (i, r) in ROUTES_URLS.iter().enumerate() {
                    let n = mine.matches(r).unwrap();
                    assert_eq!(n.data.value, i);
                }
            });
        })
        .bench_function("actix_router", |b| {
            let mut router = ActixRouter::<usize>::build();
            for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
                router.path(*r, i);
            }
            let router = router.finish();
            b.iter(|| {
                for (i, r) in ROUTES_URLS.iter().enumerate() {
                    let mut path = ActixPath::new(*r);
                    let n = router.recognize(&mut path).unwrap();
                    assert_eq!(*n.0, i);
                }
            });
        })
        .bench_function("ntex_router", |b| {
            let mut router = NtexRouter::<usize>::build();
            for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
                router.path(*r, i);
            }
            let router = router.finish();
            b.iter(|| {
                for (i, r) in ROUTES_URLS.iter().enumerate() {
                    let mut path = NtexPath::new(*r);
                    let n = router.recognize(&mut path).unwrap();
                    assert_eq!(*n.0, i);
                }
            });
        })
        .bench_function("path_table", |b| {
            let mut table: PathTable<usize> = PathTable::new();
            for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
                *table.setup(r) = i;
            }
            b.iter(|| {
                for (i, r) in ROUTES_URLS.iter().enumerate() {
                    let n = table.route(r).unwrap();
                    assert_eq!(*n.0, i);
                }
            });
        })
        .bench_function("path_tree", |b| {
            let mut tree: PathTree<usize> = PathTree::new();
            for (i, r) in ROUTES_WITH_COLON.iter().enumerate() {
                let _ = tree.insert(r, i);
            }
            b.iter(|| {
                for (i, r) in ROUTES_URLS.iter().enumerate() {
                    let n = tree.find(r).unwrap();
                    assert_eq!(*n.0, i);
                }
            });
        })
        .bench_function("matchit", |b| {
            let mut matcher = MatchitRouter::new();
            for (i, r) in ROUTES_WITH_BRACES.iter().enumerate() {
                let _ = matcher.insert(*r, i);
            }
            b.iter(|| {
                for (i, r) in ROUTES_URLS.iter().enumerate() {
                    let n = matcher.at(r).unwrap();
                    assert_eq!(*n.value, i);
                }
            });
        })
        .bench_function("route_recognizer", |b| {
            let mut router = RRRouter::<usize>::new();
            for (i, r) in ROUTES_WITH_COLON.iter().enumerate() {
                router.add(r, i);
            }
            b.iter(|| {
                for (i, r) in ROUTES_URLS.iter().enumerate() {
                    let n = router.recognize(r).unwrap();
                    assert_eq!(**n.handler(), i);
                }
            });
        })
        .sample_size(12);

    group.finish();
}

criterion_group!(benches, bench_path_find);
criterion_main!(benches);
