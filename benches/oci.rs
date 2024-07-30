use divan::{AllocProfiler, Bencher};

#[path = "../fixtures/oci.rs"]
mod oci;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

// #[divan::bench]
// fn insert(bencher: Bencher) {
//     bencher.bench_local(|| {
//         let mut mine = router::trie::Trie::new();
//         for route in oci::ROUTES {
//             mine.insert(route, 0);
//         }
//     });
// }
