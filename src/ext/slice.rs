use crate::prelude::*;

/// Extended functionality for slices.
pub trait SliceExt<A> {
    /// Returns a random shared reference in `self` without `rng` failing.
    fn get_rand<'a, R: ?Sized + Rng>(&'a self, rng: &mut R) -> Option<&'a A>;

    /// Returns a random shared reference in `self`, returning an error if `rng`
    /// fails.
    fn try_get_rand<'a, R: ?Sized + TryRng>(&'a self, rng: &mut R) -> Result<Option<&'a A>, R::Error>;

    /// Returns a random mutable reference in `self` without `rng` failing.
    fn get_rand_mut<'a, R: ?Sized + Rng>(&'a mut self, rng: &mut R) -> Option<&'a mut A>;

    /// Returns a random mutable reference in `self`, returning an error if
    /// `rng` fails.
    fn try_get_rand_mut<'a, R: ?Sized + TryRng>(&'a mut self, rng: &mut R) -> Result<Option<&'a mut A>, R::Error>;

    /// Returns a random shared reference in `self` without `rng` failing and
    /// without checking whether `self` is empty.
    unsafe fn get_rand_unchecked<'a, R: ?Sized + Rng>(&'a self, rng: &mut R) -> &'a A;

    /// Returns a random shared reference in `self` without checking whether
    /// `self` is empty, returning an error if `rng` fails.
    unsafe fn try_get_rand_unchecked<'a, R: ?Sized + TryRng>(&'a self, rng: &mut R) -> Result<&'a A, R::Error>;

    /// Returns a random mutable reference in `self` without `rng` failing and
    /// without checking whether `self` is empty.
    unsafe fn get_rand_mut_unchecked<'a, R: ?Sized + Rng>(&'a mut self, rng: &mut R) -> &'a mut A;

    /// Returns a random mutable reference in `self` without checking whether
    /// `self` is empty, returning an error if `rng` fails.
    unsafe fn try_get_rand_mut_unchecked<'a, R: ?Sized + TryRng>(&'a mut self, rng: &mut R) -> Result<&'a mut A, R::Error>;
}

impl<A> SliceExt<A> for [A] {
    #[inline]
    fn get_rand<'a, R: ?Sized + Rng>(&'a self, rng: &mut R) -> Option<&'a A> {
        <&A>::rand_in(rng, self)
    }

    #[inline]
    fn try_get_rand<'a, R: ?Sized + TryRng>(&'a self, rng: &mut R) -> Result<Option<&'a A>, R::Error> {
        <&A>::try_rand_in(rng, self)
    }

    #[inline]
    fn get_rand_mut<'a, R: ?Sized + Rng>(&'a mut self, rng: &mut R) -> Option<&'a mut A> {
        <&mut A>::rand_in(rng, self)
    }

    #[inline]
    fn try_get_rand_mut<'a, R: ?Sized + TryRng>(&'a mut self, rng: &mut R) -> Result<Option<&'a mut A>, R::Error> {
        <&mut A>::try_rand_in(rng, self)
    }

    #[inline]
    unsafe fn get_rand_unchecked<'a, R: ?Sized + Rng>(&'a self, rng: &mut R) -> &'a A {
        <&A>::rand_in_unchecked(rng, self)
    }

    #[inline]
    unsafe fn try_get_rand_unchecked<'a, R: ?Sized + TryRng>(&'a self, rng: &mut R) -> Result<&'a A, R::Error> {
        <&A>::try_rand_in_unchecked(rng, self)
    }

    #[inline]
    unsafe fn get_rand_mut_unchecked<'a, R: ?Sized + Rng>(&'a mut self, rng: &mut R) -> &'a mut A {
        <&mut A>::rand_in_unchecked(rng, self)
    }

    #[inline]
    unsafe fn try_get_rand_mut_unchecked<'a, R: ?Sized + TryRng>(&'a mut self, rng: &mut R) -> Result<&'a mut A, R::Error> {
        <&mut A>::try_rand_in_unchecked(rng, self)
    }
}
