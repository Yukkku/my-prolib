pub trait Radix: Ord + Copy {
    fn dist(&self, r: &Self) -> u32;

    const MAX: Self;
    const BITS: u32;
}

macro_rules! ui {
    ($($t: ty),*) => ($(
        impl Radix for $t {
            #[inline]
            fn dist(&self, r: &$t) -> u32 {
                <$t>::BITS - (self ^ r).leading_zeros()
            }

            const MAX: $t = <$t>::MAX;
            const BITS: u32 = <$t>::BITS;
        }
    )*);
}

ui!{ u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

pub struct RadixHeap<T> {
    s: usize,
    m: T,
    l: T,
    d: Box<[Vec<T>]>,
}

impl<T> RadixHeap<T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.s
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.s == 0
    }

    #[inline]
    pub fn peek(&self) -> Option<&T> {
        if self.s == 0 {
            None
        } else {
            Some(&self.m)
        }
    }

    #[inline]
    pub unsafe fn peek_unchecked(&self) -> &T {
        &self.m
    }
}

impl<T: Radix> RadixHeap<T> {
    pub fn new() -> Self {
        let mut d = Vec::with_capacity(T::BITS as usize + 1);
        for _ in 0..=T::BITS {
            d.push(vec![]);
        }

        Self {
            s: 0,
            m: T::MAX,
            l: T::MAX,
            d: d.into(),
        }
    }

    pub fn push(&mut self, mut v: T) {
        debug_assert!(v <= self.l);

        if self.s == 0 {
            self.m = v;
            self.s = 1;
            return;
        }

        self.s += 1;
        if v > self.m {
            (self.m, v) = (v, self.m);
        }
        self.d[self.l.dist(&v) as usize].push(v);
    }

    #[inline]
    pub fn try_push(&mut self, v: T) -> bool {
        if v <= self.l {
            self.push(v);
            true
        } else {
            false
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.s == 0 {
            return None;
        }

        self.s -= 1;

        if self.s == 0 {
            let v = Some(self.m);
            self.l = T::MAX;
            self.m = T::MAX;
            return v;
        }

        let v = self.l.dist(&self.m);
        self.l = self.m;
        if self.d[v as usize].is_empty() {'a: {
            for g in (v + 1)..=T::BITS {
                let g = g as usize;
                let r = &mut self.d[g];
                if r.is_empty() {
                    continue;
                }
                let mut max = r.pop().unwrap();
                for v in r.iter_mut() {
                    if *v > max {
                        std::mem::swap(&mut max, v);
                    }
                }
                self.m = max;
                break 'a;
            }
            unsafe { std::hint::unreachable_unchecked() }
        }} else {
            let mut r = vec![];
            std::mem::swap(&mut r, &mut self.d[v as usize]);
            let mut max = r.pop().unwrap();
            for mut v in r {
                if v > max {
                    std::mem::swap(&mut max, &mut v);
                }
                let l = self.l.dist(&v) as usize;
                self.d[l].push(v);
            }
            self.m = max;
        }
        Some(self.l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radix_heap_works() {
        let mut h = RadixHeap::new();
        h.push(3);
        h.push(0);
        h.push(-2);
        h.push(3);
        h.push(-5);
        h.push(138);
        h.push(-34);
        h.push(61);
        h.push(-213);
        h.push(45);

        assert_eq!(h.pop(), Some(138));
        assert_eq!(h.pop(), Some(61));
        assert_eq!(h.pop(), Some(45));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(0));
        assert_eq!(h.pop(), Some(-2));
        assert_eq!(h.pop(), Some(-5));
        assert_eq!(h.pop(), Some(-34));
        assert_eq!(h.pop(), Some(-213));
        assert_eq!(h.pop(), None);
    }
}
