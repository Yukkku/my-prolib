use std::ops::{Add, AddAssign};

const SEED: u64 = 314159265358979323;
const MOD: u64 = (1 << 61) - 1;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RollingHash {
    plen: u64,
    hash: u64,
}

impl RollingHash {
    pub fn new(v: u32) -> Self {
        Self {
            plen: SEED,
            hash: v as u64,
        }
    }
}

impl Default for RollingHash {
    #[inline]
    fn default() -> Self {
        Self { plen: 1, hash: 0 }
    }
}

impl AddAssign for RollingHash {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.hash += mul(self.plen, rhs.hash);
        if self.hash > MOD {
            self.hash -= MOD;
        }
        self.plen = mul(self.plen, rhs.plen);
    }
}

impl Add for RollingHash {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

#[inline]
fn mul(a: u64, b: u64) -> u64 {
    let k = (a as u128) * (b as u128);
    let k = (k >> 61) as u64 + (k as u64 & MOD);
    if k > MOD {
        k - MOD
    } else {
        k
    }
}
