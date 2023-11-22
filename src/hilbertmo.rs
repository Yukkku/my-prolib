fn hilbert_ord(mut a: (u32, u32)) -> u64 {
    let mut v = u32::BITS;
    let mut d = 0;
    while v != 0 {
        v -= 1;
        let x = (a.0 >> v) & 1 == 1;
        let y = (a.1 >> v) & 1 == 1;
        d += ((x as u64 * 3) ^ y as u64) << (v * 2);
        if !y {
            if x {
                a = (!a.1, !a.0);
            } else {
                a = (a.1, a.0);
            }
        }
    }
    d
}

pub trait Moable {
    type T;
    type U;

    fn origin() -> Self::T;
    fn convert(v: &Self::T) -> Self::U;
    fn inc_l(&self, r: (u32, u32), v: &mut Self::T);
    fn inc_r(&self, r: (u32, u32), v: &mut Self::T);
    fn dec_l(&self, r: (u32, u32), v: &mut Self::T);
    fn dec_r(&self, r: (u32, u32), v: &mut Self::T);
}

use std::collections::HashMap;
use std::hash::Hash;

pub struct Pid<'a, T: Copy + Eq + Hash> {
    v: &'a [T],
}

impl<'a, T: Copy + Eq + Hash> Pid<'a, T> {
    pub fn new(v: &'a [T]) -> Self {
        Pid { v }
    }
}

impl<'a, T: Copy + Eq + Hash> Moable for Pid<'a, T> {
    type T = HashMap<T, usize>;

    type U = usize;

    fn origin() -> Self::T {
        HashMap::new()
    }

    fn convert(v: &Self::T) -> usize {
        v.len()
    }

    fn inc_l(&self, r: (u32, u32), v: &mut HashMap<T, usize>) {
        let k = self.v[r.0 as usize];
        let mut flg = false;
        if let Some(e) = v.get_mut(&k) {
            *e -= 1;
            flg = *e == 0;
        }
        if flg {
            v.remove(&k);
        }
    }

    fn inc_r(&self, r: (u32, u32), v: &mut HashMap<T, usize>) {
        let k = self.v[r.1 as usize];
        if let Some(e) = v.get_mut(&k) {
            *e += 1;
        } else {
            v.insert(k, 1);
        }
    }

    fn dec_l(&self, r: (u32, u32), v: &mut HashMap<T, usize>) {
        let k = self.v[r.0 as usize - 1];
        if let Some(e) = v.get_mut(&k) {
            *e += 1;
        } else {
            v.insert(k, 1);
        }
    }

    fn dec_r(&self, r: (u32, u32), v: &mut HashMap<T, usize>) {
        let k = self.v[r.1 as usize - 1];
        let mut flg = false;
        if let Some(e) = v.get_mut(&k) {
            *e -= 1;
            flg = *e == 0;
        }
        if flg {
            v.remove(&k);
        }
    }
}

pub fn mo<M: Moable>(m: M, rs: &[(u32, u32)]) -> Box<[M::U]> {
    let mut x = (0..rs.len()).collect::<Vec<_>>();
    x.sort_by_cached_key(|v| hilbert_ord(rs[*v].clone()));
    let mut r = Vec::with_capacity(rs.len());
    let e = r.spare_capacity_mut();

    let mut g = M::origin();
    let mut t = (0, 0);
    for i in x {
        let p = rs[i];
        while t.1 < p.1 {
            m.inc_r(t, &mut g);
            t.1 += 1;
        }
        while t.0 > p.0 {
            m.dec_l(t, &mut g);
            t.0 -= 1;
        }
        while t.1 > p.1 {
            m.dec_r(t, &mut g);
            t.1 -= 1;
        }
        while t.0 < p.0 {
            m.inc_l(t, &mut g);
            t.0 += 1;
        }
        e[i].write(M::convert(&g));
    }
    unsafe { r.set_len(rs.len()) }
    r.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ヒルベルト曲線へのプロットが上手くいくか
    #[test]
    fn hilbert() {
        let ans = [
            [0, 3, 4, 5, 58, 59, 60, 63],
            [1, 2, 7, 6, 57, 56, 61, 62],
            [14, 13, 8, 9, 54, 55, 50, 49],
            [15, 12, 11, 10, 53, 52, 51, 48],
            [16, 17, 30, 31, 32, 33, 46, 47],
            [19, 18, 29, 28, 35, 34, 45, 44],
            [20, 23, 24, 27, 36, 39, 40, 43],
            [21, 22, 25, 26, 37, 38, 41, 42],
        ];
        for i in 0..8 {
            for j in 0..8 {
                assert_eq!(hilbert_ord((i, j)), ans[i as usize][j as usize]);
            }
        }
    }

    #[test]
    fn it_works() {
        let g = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7];
        println!(
            "{:?}",
            mo(
                Pid::new(&g),
                &[(1, 2), (5, 10), (5, 11), (4, 7), (6, 12), (5, 12), (5, 13),]
            )
        );
    }
}
