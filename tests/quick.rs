

#[macro_use]
extern crate quickcheck;

extern crate rawslice;

use rawslice::SliceIter;


// SliceIter
quickcheck! {
    fn slice_iter_find(v: Vec<i8>, offset: u8, pat: i8) -> bool {
        // use offset for a random alignment of the data
        if v.len() == 0 {
            return true;
        }
        let offset = offset as usize % v.len();
        let data = &v[offset..];

        data.iter().find(|x| **x == pat) ==
            SliceIter::from(data).find(|x| **x == pat)
    }

    fn slice_iter_position(v: Vec<i8>, offset: u8, pat: i8) -> bool {
        // use offset for a random alignment of the data
        if v.len() == 0 {
            return true;
        }
        let offset = offset as usize % v.len();
        let data = &v[offset..];

        data.iter().position(|x| *x == pat) ==
            SliceIter::from(data).position(|x| *x == pat)
    }

    fn slice_iter_rposition(v: Vec<i8>, offset: u8, pat: i8) -> bool {
        // use offset for a random alignment of the data
        if v.len() == 0 {
            return true;
        }
        let offset = offset as usize % v.len();
        let data = &v[offset..];

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
