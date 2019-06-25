use std::{fs::File, io};
use crate::rng::{TryRng, io::ReadRng};

/// Random data sourced from `/dev/random`.
#[derive(Debug)]
pub struct DevRandomRng(File);

impl DevRandomRng {
    /// Opens `/dev/random` for reading.
    #[inline]
    pub fn new() -> io::Result<Self> {
        File::open("/dev/random").map(DevRandomRng)
    }
}

/// Random data sourced from `/dev/urandom`.
#[derive(Debug)]
pub struct DevURandomRng(File);

impl DevURandomRng {
    /// Opens `/dev/urandom` for reading.
    #[inline]
    pub fn new() -> io::Result<Self> {
        File::open("/dev/urandom").map(DevURandomRng)
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

impl_try_rng!(DevRandomRng, DevURandomRng);
