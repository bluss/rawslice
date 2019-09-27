
#[macro_use] extern crate bencher;
extern crate rawslice;

use bencher::Bencher;
use bencher::black_box;

use rawslice::SliceIter;

const LEN: usize = 1024 * 1024;

fn all0_default(b: &mut Bencher) {
    let mut v = vec![0u8; LEN];
    v[LEN - 1] = black_box(1);
    b.iter(|| {
        SliceIter::from(&v[..]).all(|&x| x == 0)
    });
    b.bytes = v.len() as u64;
}

fn all0_unrolled(b: &mut Bencher) {
    let mut v = vec![0u8; LEN];
    v[LEN - 1] = black_box(1);
    b.iter(|| {
        SliceIter::from(&v[..]).unrolled().all(|&x| x == 0)
    });
    b.bytes = v.len() as u64;
}

fn position_default(b: &mut Bencher) {
    let mut v = vec![0u8; LEN];
    v[LEN - 1] = black_box(1);
    b.iter(|| {
        SliceIter::from(&v[..]).position(|&x| x != 0)
    });
    b.bytes = v.len() as u64;
}

fn position_unrolled(b: &mut Bencher) {
    let mut v = vec![0u8; LEN];
    v[LEN - 1] = black_box(1);
    b.iter(|| {
        SliceIter::from(&v[..]).unrolled().position(|&x| x != 0)
    });
    b.bytes = v.len() as u64;
}

benchmark_group!(benches,
                 all0_default,
                 all0_unrolled,
                 position_default,
                 position_unrolled
);

benchmark_main!(benches);
