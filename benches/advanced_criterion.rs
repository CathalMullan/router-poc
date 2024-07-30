use criterion::Criterion;

#[path = "../fixtures/advanced.rs"]
mod advanced;

criterion::criterion_main!(benches);
criterion::criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = routers
);

fn routers(c: &mut Criterion) {
    let mut group = c.benchmark_group("Advanced Routing");

    let mut mine = router::trie::Trie::new();
    for route in advanced::ROUTES {
        mine.insert(route, 0);
    }

    group.bench_function("mine", |b| {
        b.iter(|| {
            for search in advanced::SEARCHES {
                criterion::black_box(mine.matches(search).unwrap());
            }
        });
    });

    let regex_set = regex::bytes::RegexSet::new(advanced::ROUTES_REGEX).unwrap();

    group.bench_function("regex", |b| {
        b.iter(|| {
            for search in advanced::SEARCHES {
                criterion::black_box(regex_set.matches(search.as_bytes()));
            }
        });
    });

    group.finish();
}
