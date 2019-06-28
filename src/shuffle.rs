use crate::prelude::*;

/// A type that can be shuffled via a random number generator.
pub trait Shuffle {
    /// Shuffles `self` in-place without fail.
    fn shuffle<R: ?Sized + Rng>(&mut self, rng: &mut R);

    /// Shuffles `self` in-place, returning an error if `rng` fails at any point.
    fn try_shuffle<R: ?Sized + TryRng>(&mut self, rng: &mut R) -> Result<(), R::Error>;
}

impl<A> Shuffle for [A] {
    #[inline]
    fn shuffle<R: ?Sized + Rng>(&mut self, rng: &mut R) {
        match self.try_shuffle(rng) {
            Ok(()) => {},
            Err(err) => match err {},
        }
    }

    #[inline]
    fn try_shuffle<R: ?Sized + TryRng>(&mut self, rng: &mut R) -> Result<(), R::Error> {
        for i in 0..self.len() {
            let j = unsafe {
                // Safe because it will always return a value within the range
                usize::try_rand_in_unchecked(rng, 0..self.len())?
            };
            self.swap(i, j);
        }
        Ok(())
    }
}
