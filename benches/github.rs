use divan::{AllocProfiler, Bencher};

#[path = "../fixtures/github.rs"]
mod github;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

// #[divan::bench]
// fn insert(bencher: Bencher) {
//     bencher.bench_local(|| {
//         let mut mine = router::trie::Trie::new();
//         for route in github::ROUTES {
//             mine.insert(route, 0);
//         }
//     });
// }
