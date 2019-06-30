use core::ops::Range;
use crate::prelude::*;

/// A type that can have a random instance retrieved from an instance of `A`.
pub trait RandIn<A>: Sized {
    /// Creates an instance of `self` from within `value` using `rng` without
    /// fail, or returns `None` if `value` is empty.
    fn rand_in<R: ?Sized + Rng>(rng: &mut R, value: A) -> Option<Self>;

    /// Creates an instance of `self` from within `value` using `rng`, returning
    /// `Ok(None)` if `value` is empty or `Err` if `rng` failed.
    fn try_rand_in<R: ?Sized + TryRng>(rng: &mut R, value: A) -> Result<Option<Self>, R::Error>;

    /// Creates an instance of `self` from within `value` using `rng` without
    /// fail and without checking if `value` is empty or if `rng` returned a
    /// valid value that can be used to fetch from `value`.
    unsafe fn rand_in_unchecked<R: ?Sized + Rng>(rng: &mut R, value: A) -> Self;

    /// Creates an instance of `self` from within `value` using `rng` without
    /// checking if `value` is empty or if `rng` returned a valid value that can
    /// be used to fetch from `value`, but returning `Err` if `rng` failed.
    unsafe fn try_rand_in_unchecked<R: ?Sized + TryRng>(rng: &mut R, value: A) -> Result<Self, R::Error>;
}

impl<'a, A> RandIn<&'a [A]> for &'a A {
    #[inline]
    fn rand_in<R: ?Sized + Rng>(rng: &mut R, slice: &'a [A]) -> Option<Self> {
        if slice.is_empty() {
            None
        } else {
            unsafe { Some(Self::rand_in_unchecked(rng, slice)) }
        }
    }

    #[inline]
    fn try_rand_in<R: ?Sized + TryRng>(rng: &mut R, slice: &'a [A]) -> Result<Option<Self>, R::Error> {
        if slice.is_empty() {
            Ok(None)
        } else {
            unsafe { Self::try_rand_in_unchecked(rng, slice).map(Some) }
        }
    }

    #[inline]
    unsafe fn rand_in_unchecked<R: ?Sized + Rng>(rng: &mut R, slice: &'a [A]) -> Self {
        // Index is safe because it will always return a value within the range
        let index = usize::rand_in_unchecked(rng, 0..slice.len());
        slice.get_unchecked(index)
    }

    #[inline]
    unsafe fn try_rand_in_unchecked<R: ?Sized + TryRng>(rng: &mut R, slice: &'a [A]) -> Result<Self, R::Error> {
        // Index is safe because it will always return a value within the range
        let index = usize::try_rand_in_unchecked(rng, 0..slice.len())?;
        Ok(slice.get_unchecked(index))
    }
}

impl<'a, A> RandIn<&'a mut [A]> for &'a mut A {
    #[inline]
    fn rand_in<R: ?Sized + Rng>(rng: &mut R, slice: &'a mut [A]) -> Option<Self> {
        if slice.is_empty() {
            None
        } else {
            unsafe { Some(Self::rand_in_unchecked(rng, slice)) }
        }
    }

    #[inline]
    fn try_rand_in<R: ?Sized + TryRng>(rng: &mut R, slice: &'a mut [A]) -> Result<Option<Self>, R::Error> {
        if slice.is_empty() {
            Ok(None)
        } else {
            unsafe { Self::try_rand_in_unchecked(rng, slice).map(Some) }
        }
    }

    #[inline]
    unsafe fn rand_in_unchecked<R: ?Sized + Rng>(rng: &mut R, slice: &'a mut [A]) -> Self {
        // Index is safe because it will always return a value within the range
        let index = usize::rand_in_unchecked(rng, 0..slice.len());
        slice.get_unchecked_mut(index)
    }

    #[inline]
    unsafe fn try_rand_in_unchecked<R: ?Sized + TryRng>(rng: &mut R, slice: &'a mut [A]) -> Result<Self, R::Error> {
        // Index is safe because it will always return a value within the range
        let index = usize::try_rand_in_unchecked(rng, 0..slice.len())?;
        Ok(slice.get_unchecked_mut(index))
    }
}

macro_rules! impl_i {
    ($($int:ty),+) => { $(
        impl RandIn<Range<$int>> for $int {
            #[inline]
            fn rand_in<R: ?Sized + Rng>(rng: &mut R, range: Range<$int>) -> Option<Self> {
                unimplemented!()
            }

            #[inline]
            fn try_rand_in<R: ?Sized + TryRng>(rng: &mut R, range: Range<$int>) -> Result<Option<Self>, R::Error> {
                unimplemented!()
            }

            #[inline]
            unsafe fn rand_in_unchecked<R: ?Sized + Rng>(rng: &mut R, range: Range<$int>) -> Self {
                unimplemented!()
            }

            #[inline]
            unsafe fn try_rand_in_unchecked<R: ?Sized + TryRng>(rng: &mut R, range: Range<$int>) -> Result<Self, R::Error> {
                unimplemented!()
            }
        }
    )+ }
}

macro_rules! impl_u {
    ($($int:ty),+) => { $(
        impl RandIn<Range<$int>> for $int {
            #[inline]
            fn rand_in<R: ?Sized + Rng>(rng: &mut R, range: Range<$int>) -> Option<Self> {
                unimplemented!()
            }

            #[inline]
            fn try_rand_in<R: ?Sized + TryRng>(rng: &mut R, range: Range<$int>) -> Result<Option<Self>, R::Error> {
                unimplemented!()
            }

            #[inline]
            unsafe fn rand_in_unchecked<R: ?Sized + Rng>(rng: &mut R, range: Range<$int>) -> Self {
                unimplemented!()
            }

            #[inline]
            unsafe fn try_rand_in_unchecked<R: ?Sized + TryRng>(rng: &mut R, range: Range<$int>) -> Result<Self, R::Error> {
                unimplemented!()
            }
        }
    )+ }
}

impl_i!(i8, i16, i32, i64, i128, isize);
impl_u!(u8, u16, u32, u64, u128, usize);
