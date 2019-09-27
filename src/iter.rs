//! Slice iterators

use std::mem::size_of;
use std::marker::PhantomData;
use std::ops::Index;
use std::slice;
use std::slice::{Iter as CoreSliceIter};
use std::ptr::NonNull;

use rawpointer::PointerExt;
use rawpointer::ptrdistance;


/// Slice (contiguous data) iterator.
///
/// Iterator element type is `&T`
///
/// This iterator exists mainly to have the constructor from a pair
/// of raw pointers available, which the libcore slice iterator does not allow.
///
/// The SliceIter's element searching methods `all, find, position, rposition`
/// are explicitly unrolled so that they often perform better than the libcore
/// slice iterator's variants of those.
///
/// **Extra Features:**
///
/// + unrolled `all, find, position, rposition`,
/// + accessors (incl. mutable) of start, end pointers
/// + construct from raw pointers
/// + native `peek_next`
/// + native `next_unchecked`.
/// + implement `Copy`, `Index`, `Default`
///
/// Notice that we don't have access to or use all the unstable features
/// libcore can use, so some of the perks of the libcore slice iterator
/// are missing.
///
/// **Missing Features:**
///
/// + No `TrustedRandomAccess` or `TrustedLen` (unstable features)
/// + No `std::intrinsics::assume`.
/// + No support for zero-sized iterator element type
#[derive(Debug)]
pub struct SliceIter<'a, T: 'a> {
    ptr: NonNull<T>,
    end: NonNull<T>,
    ty: PhantomData<&'a T>,
}

impl<'a, T> Copy for SliceIter<'a, T> { }
impl<'a, T> Clone for SliceIter<'a, T> {
    fn clone(&self) -> Self { *self }
}

// Same bound as std::slice::Iter
unsafe impl<'a, T> Send for SliceIter<'a, T> where T: Sync { }

unsafe fn nonnull<T>(p: *const T) -> NonNull<T> {
    debug_assert!(!p.is_null());
    NonNull::new_unchecked(p as _)
}

impl<'a, T> SliceIter<'a, T> {
    /// Create a new slice iterator
    ///
    /// See also ``SliceIter::from, SliceIter::default``.
    ///
    /// Panics if `T` is a zero-sized type. That case is not supported.
    #[inline]
    pub unsafe fn new(start: *const T, end: *const T) -> Self {
        assert!(size_of::<T>() != 0);
        SliceIter {
            ptr: nonnull(start),
            end: nonnull(end),
            ty: PhantomData,
        }
    }

    /// Return the start pointer
    pub fn start(&self) -> *const T {
        self.ptr.as_ptr() as _
    }

    /// Return the end pointer
    pub fn end(&self) -> *const T {
        self.end.as_ptr() as _
    }

    /// Return mutable reference to the start pointer
    ///
    /// Unsafe because it is easy to violate memory safety by setting
    /// the pointer outside the data's valid range.
    pub unsafe fn start_mut(&mut self) -> &mut *const T {
        panic!()
        //&mut self.ptr
    }

    /// Return mutable reference to the start pointer
    ///
    /// Unsafe because it is easy to violate memory safety by setting
    /// the pointer outside the data's valid range.
    pub unsafe fn end_mut(&mut self) -> &mut *const T {
        panic!()
        //&mut self.end
    }

    /// Return the next iterator element, without stepping the iterator.
    pub fn peek_next(&self) -> Option<<Self as Iterator>::Item> {
        if self.ptr != self.end {
            unsafe {
                Some(&*self.ptr.as_ptr())
            }
        } else {
            None
        }
    }

    /// Return the equivalent slice
    pub fn as_slice(&self) -> &'a [T] {
        unsafe {
            slice::from_raw_parts(self.ptr.as_ptr(), self.len())
        }
    }

    /// Return the next iterator element, without checking if the end is reached
    #[inline]
    pub unsafe fn next_unchecked(&mut self) -> <Self as Iterator>::Item {
        &*self.ptr.post_inc().as_ptr()
    }

    /// Return a reference to the element at `i`.
    pub unsafe fn get_unchecked(&self, i: usize) -> &T {
        &*self.ptr.as_ptr().add(i)
    }
}

impl<'a, T> Iterator for SliceIter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr != self.end {
            unsafe {
                Some(&*self.ptr.post_inc().as_ptr())
            }
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    fn all<F>(&mut self, mut predicate: F) -> bool
        where F: FnMut(Self::Item) -> bool,
    {
        self.fold_while(true, move |_, elt| {
            if predicate(elt) {
                FoldWhile::Continue(true)
            } else {
                FoldWhile::Done(false)
            }
        })
    }

    fn any<F>(&mut self, mut predicate: F) -> bool
        where F: FnMut(Self::Item) -> bool,
    {
        !self.all(move |x| !predicate(x))
    }

    fn find<F>(&mut self, mut predicate: F) -> Option<Self::Item>
        where F: FnMut(&Self::Item) -> bool,
    {
        self.fold_while(None, move |_, elt| {
            if predicate(&elt) {
                FoldWhile::Done(Some(elt))
            } else {
                FoldWhile::Continue(None)
            }
        })
    }

    fn position<F>(&mut self, mut predicate: F) -> Option<usize>
        where F: FnMut(Self::Item) -> bool,
    {
        let mut index = 0;
        self.fold_while(None, move |_, elt| {
            if predicate(elt) {
                FoldWhile::Done(Some(index))
            } else {
                index += 1;
                FoldWhile::Continue(None)
            }
        })
    }

    fn rposition<F>(&mut self, mut predicate: F) -> Option<usize>
        where F: FnMut(Self::Item) -> bool,
    {
        let mut index = self.len();
        self.rfold_while(None, move |_, elt| {
            index -= 1;
            if predicate(elt) {
                FoldWhile::Done(Some(index))
            } else {
                FoldWhile::Continue(None)
            }
        })
    }
}

impl<'a, T> DoubleEndedIterator for SliceIter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.ptr != self.end {
            unsafe {
                Some(&*self.end.pre_dec().as_ptr())
            }
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for SliceIter<'a, T> {
    fn len(&self) -> usize {
        ptrdistance(self.ptr.as_ptr(), self.end.as_ptr())
    }
}

impl<'a, T> From<&'a [T]> for SliceIter<'a, T> {
    fn from(slice: &'a [T]) -> Self {
        unsafe {
            let ptr = slice.as_ptr();
            let end = ptr.add(slice.len());
            SliceIter::new(ptr, end)
        }
    }
}

impl<'a, T> From<CoreSliceIter<'a, T>> for SliceIter<'a, T> {
    fn from(slice: CoreSliceIter<'a, T>) -> Self {
        SliceIter::from(slice.as_slice())
    }
}

impl<'a, T> Default for SliceIter<'a, T> {
    /// Create an empty `SliceIter`.
    fn default() -> Self {
        unsafe {
            SliceIter::new(0x1 as *const T, 0x1 as *const T)
        }
    }
}

impl<'a, T> Index<usize> for SliceIter<'a, T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        assert!(i < self.len());
        unsafe {
            &*self.ptr.add(i).as_ptr()
        }
    }
}



// Fold while implements unrolled searching

#[derive(Copy, Clone, Debug)]
/// An enum used for controlling the execution of `.fold_while()`.
enum FoldWhile<T> {
    /// Continue folding with this value
    Continue(T),
    /// Fold is complete and will return this value
    Done(T),
}

trait FoldWhileExt : Iterator {
    // Note: For composability (if used with adaptors, return type
    // should be FoldWhile<Acc> then instead.)
    fn fold_while<Acc, G>(&mut self, init: Acc, g: G) -> Acc
        where Self: Sized,
              G: FnMut(Acc, Self::Item) -> FoldWhile<Acc>;
    fn rfold_while<Acc, G>(&mut self, accum: Acc, g: G) -> Acc
        where Self: Sized,
              G: FnMut(Acc, Self::Item) -> FoldWhile<Acc>;
}

macro_rules! fold_while {
    ($e:expr) => {
        match $e {
            FoldWhile::Continue(t) => t,
            FoldWhile::Done(done) => return done,
        }
    }
}

impl<'a, T> FoldWhileExt for SliceIter<'a, T> {
    fn fold_while<Acc, G>(&mut self, init: Acc, mut g: G) -> Acc
        where Self: Sized,
              G: FnMut(Acc, Self::Item) -> FoldWhile<Acc>
    {

        let mut accum = init;
        unsafe {
            while self.len() >= 4 {
                accum = fold_while!(g(accum, &*self.ptr.post_inc().as_ptr()));
                accum = fold_while!(g(accum, &*self.ptr.post_inc().as_ptr()));
                accum = fold_while!(g(accum, &*self.ptr.post_inc().as_ptr()));
                accum = fold_while!(g(accum, &*self.ptr.post_inc().as_ptr()));
            }
            while self.ptr != self.end {
                accum = fold_while!(g(accum, &*self.ptr.post_inc().as_ptr()));
            }
        }
        accum
    }

    fn rfold_while<Acc, G>(&mut self, mut accum: Acc, mut g: G) -> Acc
        where Self: Sized,
              G: FnMut(Acc, Self::Item) -> FoldWhile<Acc>
    {
        // manual unrolling is needed when there are conditional exits from the loop's body.
        unsafe {
            while self.len() >= 4 {
                accum = fold_while!(g(accum, &*self.end.pre_dec().as_ptr()));
                accum = fold_while!(g(accum, &*self.end.pre_dec().as_ptr()));
                accum = fold_while!(g(accum, &*self.end.pre_dec().as_ptr()));
                accum = fold_while!(g(accum, &*self.end.pre_dec().as_ptr()));
            }
            while self.ptr != self.end {
                accum = fold_while!(g(accum, &*self.end.pre_dec().as_ptr()));
            }
        }
        accum
    }
}
