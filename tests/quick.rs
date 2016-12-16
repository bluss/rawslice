

#[macro_use]
extern crate quickcheck;

extern crate rawslice;

use rawslice::SliceIter;

const MAX_OFFSET: usize = 15;

// use offset for a random alignment of the data
fn offset<T>(v: &[T], offset: usize) -> &[T] {
    if v.len() == 0 {
        return v;
    }
    let offset = (offset % MAX_OFFSET) % v.len();
    &v[offset..]
}

// SliceIter
quickcheck! {
    fn slice_iter_find(v: Vec<i8>, off: usize, pat: i8) -> bool {
        let data = offset(&v, off);

        data.iter().find(|x| **x == pat) ==
            SliceIter::from(data).find(|x| **x == pat)
    }

    fn slice_iter_position(v: Vec<i8>, off: usize, pat: i8) -> bool {
        let data = offset(&v, off);

        data.iter().position(|x| *x == pat) ==
            SliceIter::from(data).position(|x| *x == pat)
    }

    fn slice_iter_rposition(v: Vec<i8>, off: usize, pat: i8) -> bool {
        let data = offset(&v, off);

        data.iter().rposition(|x| *x == pat) ==
            SliceIter::from(data).rposition(|x| *x == pat)
    }

    fn slice_iter_all(v: Vec<i8>) -> bool {
        v.iter().all(|x| *x == 0) == SliceIter::from(&v[..]).all(|x| *x == 0)
    }
    fn slice_iter_any(v: Vec<i8>) -> bool {
        v.iter().any(|x| *x == 0) == SliceIter::from(&v[..]).any(|x| *x == 0)
    }
}
