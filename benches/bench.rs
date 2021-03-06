#![cfg(feature = "__test_data")]

use bencher::{benchmark_group, benchmark_main, Bencher};

benchmark_group!(benches, short);
benchmark_main!(benches);

fn short(bench: &mut Bencher) {
    let segmenter = instant_segment::test_data::segmenter();
    let mut search = instant_segment::Search::default();
    bench.iter(|| {
        let _ = segmenter.segment("thisisatest", &mut search);
    });
}
