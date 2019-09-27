
#[macro_use] extern crate bencher;
extern crate rawslice;

use bencher::Bencher;
use bencher::black_box;

use rawslice::SliceIter;

const LEN: usize = 1024 * 1024;

// Wrap an iterator and provide no smarts - just the .next() method
struct WrapIter<I>(I);

impl<I> Iterator for WrapIter<I> where I: Iterator {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

fn all0_default(b: &mut Bencher) {
    let mut v = vec![0u8; LEN];
    v[LEN - 1] = black_box(1);
    b.iter(|| {
        SliceIter::from(&v[..]).all(|&x| x == 0)
    });
    b.bytes = v.len() as u64;
}

fn all0_default_wrap(b: &mut Bencher) {
    let mut v = vec![0u8; LEN];
    v[LEN - 1] = black_box(1);
    b.iter(|| {
        WrapIter(SliceIter::from(&v[..])).all(|&x| x == 0)
    });
    b.bytes = v.len() as u64;
}

fn all0_std_iter(b: &mut Bencher) {
    let mut v = vec![0u8; LEN];
    v[LEN - 1] = black_box(1);
    b.iter(|| {
        v[..].iter().all(|&x| x == 0)
    });
    b.bytes = v.len() as u64;
}

fn all0_std_iter_wrap(b: &mut Bencher) {
    let mut v = vec![0u8; LEN];
    v[LEN - 1] = black_box(1);
    b.iter(|| {
        WrapIter(v[..].iter()).all(|&x| x == 0)
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

fn position_std_iter(b: &mut Bencher) {
    let mut v = vec![0u8; LEN];
    v[LEN - 1] = black_box(1);
    b.iter(|| {
        v[..].iter().position(|&x| x != 0)
    });
    b.bytes = v.len() as u64;
}


benchmark_group!(benches,
                 all0_default,
                 all0_default_wrap,
                 all0_std_iter,
                 all0_std_iter_wrap,
                 position_default,
                 position_std_iter
);

benchmark_main!(benches);
