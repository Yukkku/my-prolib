use num::{One, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct ModInt<const N: u32> {
    v: u32,
}

impl<const N: u32> ModInt<N> {
    #[inline]
    #[must_use]
    pub const fn new(v: u32) -> Self {
        ModInt { v: v % N }
    }

    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(v: u32) -> Self {
        debug_assert!(v < N);

        ModInt { v }
    }

    #[inline]
    #[must_use]
    pub const fn val(&self) -> u32 {
        self.v
    }

    #[must_use]
    pub const fn inv(&self) -> Self {
        debug_assert!(self.v != 0);

        let mut a = self.v;
        let mut b = N;
        let mut u = 1_i64;
        let mut v = 0;
        while b != 0 {
            let t = a / b;
            a -= t * b;
            (a, b) = (b, a);
            u -= t as i64 * v;
            (u, v) = (v, u);
        }
        ModInt {
            v: u.rem_euclid(N as i64) as u32,
        }
    }

    #[must_use]
    pub const fn pow(self, mut b: u64) -> Self {
        let mut g = self.v;
        let mut r = if b & 1 == 0 { 1 } else { self.v };
        b >>= 1;
        while b != 0 {
            g = (g as u64 * g as u64 % N as u64) as u32;
            if b & 1 == 1 {
                r = (r as u64 * g as u64 % N as u64) as u32;
            }
            b >>= 1;
        }
        ModInt { v: r }
    }
}

impl<const N: u32> num::FromPrimitive for ModInt<N> {
    fn from_i64(n: i64) -> Option<Self> {
        let k = n % N as i64;
        Some(ModInt {
            v: if k < 0 {
                (k + N as i64) as u32
            } else {
                k as u32
            },
        })
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(ModInt {
            v: (n % N as u64) as u32,
        })
    }
}

impl<const N: u32> num::ToPrimitive for ModInt<N> {
    fn to_i64(&self) -> Option<i64> {
        Some(self.v as i64)
    }

    fn to_u64(&self) -> Option<u64> {
        Some(self.v as u64)
    }
}

impl<const N: u32> Zero for ModInt<N> {
    fn zero() -> Self {
        ModInt { v: 0 }
    }

    fn is_zero(&self) -> bool {
        self.v == 0
    }
}

impl<const N: u32> One for ModInt<N> {
    fn one() -> Self {
        ModInt { v: 1 }
    }
}

impl<const N: u32> std::fmt::Display for ModInt<N> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.v.fmt(f)
    }
}

impl<const N: u32> std::fmt::Debug for ModInt<N> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.v.fmt(f)
    }
}

impl<const N: u32> From<u32> for ModInt<N> {
    #[inline]
    fn from(value: u32) -> Self {
        ModInt { v: value % N }
    }
}

impl<const N: u32> Into<u32> for ModInt<N> {
    #[inline]
    fn into(self) -> u32 {
        self.v
    }
}

impl<const N: u32> Add for ModInt<N> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        let (v, f) = self.v.overflowing_add(rhs.v);
        if f || v >= N {
            ModInt {
                v: v.wrapping_sub(N),
            }
        } else {
            ModInt { v }
        }
    }
}

impl<const N: u32> Sub for ModInt<N> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        let (v, f) = self.v.overflowing_sub(rhs.v);
        if f {
            ModInt {
                v: v.wrapping_add(N),
            }
        } else {
            ModInt { v }
        }
    }
}

impl<const N: u32> Mul for ModInt<N> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        ModInt {
            v: ((self.v as u64) * (rhs.v as u64) % (N as u64)) as u32,
        }
    }
}

impl<const N: u32> Div for ModInt<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<const N: u32> Neg for ModInt<N> {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self {
        if self.v != 0 {
            self.v = N - self.v;
        }
        self
    }
}

impl<const N: u32> Neg for &ModInt<N> {
    type Output = ModInt<N>;

    #[inline]
    fn neg(self) -> ModInt<N> {
        -*self
    }
}

impl<const N: u32> std::iter::Sum for ModInt<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt::zero(), |a, b| a + b)
    }
}

impl<const N: u32> std::iter::Product for ModInt<N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt::one(), |a, b| a * b)
    }
}

macro_rules! ui {
    ($($t: ident, $i: ident, $u: ident, $j: ident),*) => ($(
        impl<const N: u32> $u for ModInt<N> {
            #[inline]
            fn $j(&mut self, rhs: Self) {
                *self = self.$i(rhs);
            }
        }

        impl<const N: u32> $t<ModInt<N>> for &ModInt<N> {
            type Output = ModInt<N>;

            #[inline]
            fn $i(self, rhs: ModInt<N>) -> ModInt<N> {
                self.clone().$i(rhs)
            }
        }
        impl<const N: u32> $t<&ModInt<N>> for ModInt<N> {
            type Output = ModInt<N>;

            #[inline]
            fn $i(self, rhs: &ModInt<N>) -> ModInt<N> {
                self.$i(rhs.clone())
            }
        }
        impl<const N: u32> $t<&ModInt<N>> for &ModInt<N> {
            type Output = ModInt<N>;

            #[inline]
            fn $i(self, rhs: &ModInt<N>) -> ModInt<N> {
                self.clone().$i(rhs.clone())
            }
        }
    )*);
}

ui! { Add, add, AddAssign, add_assign }
ui! { Sub, sub, SubAssign, sub_assign }
ui! { Mul, mul, MulAssign, mul_assign }
ui! { Div, div, DivAssign, div_assign }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modint_works() {
        type Mint = ModInt<998244353>;

        let a = Mint::new(314159265);
        let b = Mint::new(358979323);

        assert_eq!(-a, Mint::new(684085088));
        assert_eq!(a + b, Mint::new(673138588));
        assert_eq!(a - b, Mint::new(953424295));
        assert_eq!(a * b, Mint::new(525838123));
        assert_eq!(a / b, Mint::new(465339227));
    }
}
