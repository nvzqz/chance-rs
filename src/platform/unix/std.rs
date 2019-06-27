use std::{fs::File, io};
use crate::rng::{TryRng, io::ReadRng};

/// Random data sourced from `/dev/random`.
#[derive(Debug)]
pub struct DevRandom(File);

impl DevRandom {
    /// Opens `/dev/random` for reading.
    #[inline]
    pub fn new() -> io::Result<Self> {
        File::open("/dev/random").map(DevRandom)
    }
}

/// Random data sourced from `/dev/urandom`.
#[derive(Debug)]
pub struct DevURandom(File);

impl DevURandom {
    /// Opens `/dev/urandom` for reading.
    #[inline]
    pub fn new() -> io::Result<Self> {
        File::open("/dev/urandom").map(DevURandom)
    }
}

macro_rules! impl_try_rng {
    ($($t:ty),+) => { $(
        impl TryRng for $t {
            type Error = io::Error;

            #[inline]
            fn try_fill_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
                ReadRng::from_mut(&mut self.0).try_fill_bytes(buf)
            }
        }
    )+ }
}

impl_try_rng!(DevRandom, DevURandom);
