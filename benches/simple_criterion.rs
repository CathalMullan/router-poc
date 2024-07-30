use criterion::Criterion;

#[path = "../fixtures/simple.rs"]
mod simple;

criterion::criterion_main!(benches);
criterion::criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = routers
);

fn routers(c: &mut Criterion) {
    let mut group = c.benchmark_group("Simple Routing");

    let mut mine = router::trie::Trie::new();
    for route in simple::ROUTES {
        mine.insert(route, 0);
    }

    group.bench_function("mine", |b| {
        b.iter(|| {
            for search in simple::SEARCHES {
                criterion::black_box(mine.matches(search).unwrap());
            }
        });
    });

    let mut matchit = matchit::Router::new();
    for route in simple::ROUTES {
        matchit.insert(route, 0).unwrap();
    }

    group.bench_function("matchit", |b| {
        b.iter(|| {
            for search in simple::SEARCHES {
                criterion::black_box(matchit.at(search).unwrap());
            }
        });
    });

    let gonzales = gonzales::RouterBuilder::new().build(simple::ROUTES);

    group.bench_function("gonzales", |b| {
        b.iter(|| {
            for search in simple::SEARCHES {
                criterion::black_box(gonzales.route(search).unwrap());
            }
        });
    });

    group.finish();
}
