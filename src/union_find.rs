/// 素集合データ構造
///
/// * Union by Size + Path Compression
#[derive(Clone, Default)]
pub struct UnionFind {
    d: Box<[isize]>,
    c: usize,
}

impl UnionFind {
    /// {{0}, {1}, {2}, ... {n-1}} のように初期化されたUnionFindを生成する
    ///
    /// # Constraints
    ///
    /// * `n` <= `1 << (usize::BITS - 1)`
    ///
    /// # Complexity
    ///
    /// * *Θ*(*n*)
    #[must_use]
    pub fn new(n: usize) -> UnionFind {
        debug_assert!(n <= 1 << (usize::BITS - 1));

        UnionFind {
            d: vec![-1; n].into(),
            c: n,
        }
    }

    /// 集合のサイズの総和を返す
    ///
    /// # Complexity
    ///
    /// * *Θ*(1)
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.d.len()
    }

    /// 集合の数を返す
    ///
    /// # Complexity
    ///
    /// * *Θ*(1)
    #[inline]
    #[must_use]
    pub fn count(&self) -> usize {
        self.c
    }

    /// `a`が含まれる集合と`b`が含まれる集合をマージする. `a`と`b`が最初から同じ集合に属していた場合は何もしない
    ///
    /// 返り値はマージしてできた新しい集合の代表になる. `a`と`b`が最初から同じ集合に属していた場合はその集合の代表を返す
    ///
    /// # Constraints
    ///
    /// * `a`, `b` < `n`
    ///
    /// # Complexity
    ///
    /// * amortized *O*(*α*(*n*))
    /// * worst *O*(log *n*)
    pub fn union(&mut self, a: usize, b: usize) -> usize {
        debug_assert!(a < self.len());
        debug_assert!(b < self.len());

        let a = self.find(a);
        let b = self.find(b);

        if a == b {
            return a;
        }
        self.c -= 1;

        if self.d[a] < self.d[b] {
            // size(a) > size(b)
            self.d[a] += self.d[b];
            self.d[b] = a as isize;
            a
        } else {
            // size(a) <= size(b)
            self.d[b] += self.d[a];
            self.d[a] = b as isize;
            b
        }
    }

    /// `a`が含まれる集合の代表を探す
    ///
    /// # Constraints
    ///
    /// * `a` < `n`
    ///
    /// # Complexity
    ///
    /// * amortized *O*(*α*(*n*))
    /// * worst *O*(log *n*)
    #[must_use]
    pub fn find(&mut self, a: usize) -> usize {
        debug_assert!(a < self.len());

        let mut r = a;
        while self.d[r] >= 0 {
            r = self.d[r] as usize;
        }
        let k = r as isize;
        let mut f = a;
        while self.d[f] >= 0 {
            let t = self.d[f] as usize;
            self.d[f] = k;
            f = t;
        }
        r
    }

    /// [`find`](UnionFind::find)を不変なオブジェクトに対して行う
    ///
    /// # Constraints
    ///
    /// * `a` < `n`
    ///
    /// # Complexity
    ///
    /// * worst *O*(log *n*)
    #[must_use]
    pub fn find_imu(&self, mut a: usize) -> usize {
        debug_assert!(a < self.len());

        while self.d[a] >= 0 {
            a = self.d[a] as usize;
        }
        a
    }

    /// `a`と`b`が同じ集合に属するか判定する
    ///
    /// # Constraints
    ///
    /// * `a`, `b` < `n`
    ///
    /// # Complexity
    ///
    /// * amortized *O*(*α*(*n*))
    /// * worst *O*(log *n*)
    #[inline]
    #[must_use]
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        debug_assert!(a < self.len());
        debug_assert!(b < self.len());

        self.find(a) == self.find(b)
    }

    /// [`same`](UnionFind::same)を不変なオブジェクトに対して行う
    ///
    /// # Constraints
    ///
    /// * `a`, `b` < `n`
    ///
    /// # Complexity
    ///
    /// * worst *O*(log *n*)
    #[inline]
    #[must_use]
    pub fn same_imu(&self, a: usize, b: usize) -> bool {
        debug_assert!(a < self.len());
        debug_assert!(b < self.len());

        self.find_imu(a) == self.find_imu(b)
    }

    /// `a`が含まれる集合のサイズを返す
    ///
    /// # Constraints
    ///
    /// * `a` < `n`
    ///
    /// # Complexity
    ///
    /// * amortized *O*(*α*(*n*))
    /// * worst *O*(log *n*)
    #[inline]
    #[must_use]
    pub fn size(&mut self, a: usize) -> usize {
        debug_assert!(a < self.len());

        -self.d[self.find(a)] as usize
    }

    /// [`size`](UnionFind::size)を不変なオブジェクトに対して行う
    ///
    /// # Constraints
    ///
    /// * `a` < `n`
    ///
    /// # Complexity
    ///
    /// * worst *O*(log *n*)
    #[inline]
    #[must_use]
    pub fn size_imu(&self, a: usize) -> usize {
        debug_assert!(a < self.len());

        -self.d[self.find_imu(a)] as usize
    }

    /// 集合の状態を返す
    ///
    /// # Complexity
    ///
    /// * *Θ*(*n*)
    #[must_use]
    pub fn groups(&mut self) -> Box<[Box<[usize]>]> {
        let mut v = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            let c = self.d[i];
            if c < 0 {
                let mut e = Vec::with_capacity(-c as usize);
                e.push(i);
                v.push(e);
            } else {
                v.push(vec![]);
            }
        }
        for i in 0..self.len() {
            if self.d[i] >= 0 {
                v[self.find(i)].push(i);
            }
        }
        let mut ans = Vec::with_capacity(self.c);
        for e in v.into_iter() {
            if !e.is_empty() {
                ans.push(e.into_boxed_slice());
            }
        }
        ans.into_boxed_slice()
    }

    /// [`groups`](UnionFind::groups)を不変なオブジェクトに対して行う
    ///
    /// # Complexity
    ///
    /// * worst *O*(*n* log *n*)
    #[must_use]
    pub fn groups_imu(&self) -> Box<[Box<[usize]>]> {
        let mut v = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            let c = self.d[i];
            if c < 0 {
                let mut e = Vec::with_capacity(-c as usize);
                e.push(i);
                v.push(e);
            } else {
                v.push(vec![]);
            }
        }
        for i in 0..self.len() {
            if self.d[i] >= 0 {
                v[self.find_imu(i)].push(i);
            }
        }
        let mut ans = Vec::with_capacity(self.c);
        for e in v.into_iter() {
            if !e.is_empty() {
                ans.push(e.into_boxed_slice());
            }
        }
        ans.into_boxed_slice()
    }
}

impl std::fmt::Debug for UnionFind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnionFind {{")?;
        for (i, v) in self.groups_imu().iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            f.debug_set().entries(v.iter()).finish()?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_find_works() {
        let mut uf = UnionFind::new(5);

        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.size(0), 1);

        uf.union(0, 2);
        uf.union(1, 3);

        assert_eq!(uf.count(), 3);
        assert_eq!(uf.size(0), 2);
        assert_eq!(uf.size(4), 1);
        assert!(uf.same(0, 2));
        assert!(!uf.same(0, 3));
    }
}
