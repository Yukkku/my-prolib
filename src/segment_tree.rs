use super::algebra::{Associative, Identity};

/// セグメントツリー
#[derive(Clone)]
pub struct SegmentTree<M: Associative + Identity> {
    d: Box<[M::T]>,
    l: usize,
}

impl<M: Associative + Identity, V: Into<Vec<M::T>>> From<V> for SegmentTree<M> {
    /// # Complexity
    ///
    /// * *Θ*(*n*)
    fn from(value: V) -> Self {
        let mut d: Vec<M::T> = value.into();
        if d.is_empty() {
            return SegmentTree { d: [].into(), l: 0 };
        }

        let l = d.len();
        let s = 1 << (64 - (l - 1).leading_zeros());
        d.reserve_exact((s << 1) - l - 1);

        for _ in l..s {
            d.push(M::e());
        }
        for i in 0..(s - 1) {
            d.push(M::op(&d[i << 1], &d[(i << 1) | 1]));
        }

        SegmentTree { d: d.into(), l }
    }
}

impl<M: Associative + Identity> std::ops::Deref for SegmentTree<M> {
    type Target = [M::T];

    fn deref(&self) -> &[M::T] {
        &self.d[..self.len()]
    }
}

impl<M: Associative + Identity> SegmentTree<M> {
    /// 要素の総数を返す
    ///
    /// # Complexity
    ///
    /// * *Θ*(1)
    #[inline]
    pub fn len(&self) -> usize {
        self.l
    }

    /// 空のセグ木かどうか調べる
    ///
    /// # Complexity
    ///
    /// * *Θ*(1)
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.l == 0
    }

    /// 値を変更する
    ///
    /// # Constraints
    ///
    /// * `index` < `n`
    ///
    /// # Complexity
    ///
    /// * *Θ*(*log n*)
    pub fn set(&mut self, mut index: usize, value: M::T) {
        debug_assert!(index < self.len());

        self.d[index] = value;

        let s = self.d.len();
        let l = s + 1;
        loop {
            index = (index | l) >> 1;
            if index == s {
                break;
            }
            self.d[index] = M::op(&self.d[(index << 1) & s], &self.d[((index << 1) & s) | 1]);
        }
    }

    /// 値を取得する
    ///
    /// # Constraints
    ///
    /// * `index` < `n`
    ///
    /// # Complexity
    ///
    /// * *Θ*(1)
    #[inline]
    pub fn get(&self, index: usize) -> &M::T {
        debug_assert!(index < self.len());

        &self.d[index]
    }

    /// 区間での総積を取得する
    ///
    /// # Constraints
    ///
    /// * `l` <= `r` <= `n`
    ///
    /// # Complexity
    ///
    /// * avelage *O*(*log n*)
    /// * worst *O*(*log n*)
    pub fn prod<R: std::ops::RangeBounds<usize>>(&self, range: R) -> M::T {
        let mut s = match range.start_bound() {
            std::ops::Bound::Included(&i) => i,
            std::ops::Bound::Excluded(&i) => i + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let mut e = match range.end_bound() {
            std::ops::Bound::Included(&i) => i + 1,
            std::ops::Bound::Excluded(&i) => i,
            std::ops::Bound::Unbounded => self.len(),
        };
        debug_assert!(s <= e && e <= self.len());

        let mut l = M::e();
        let mut r = M::e();

        let v = self.d.len() + 1;
        while s != e {
            if s & 1 == 1 {
                l = M::op(&l, &self.d[s]);
                s += 1;
            }
            if e & 1 == 1 {
                e -= 1;
                r = M::op(&self.d[e], &r);
            }
            s = (s | v) >> 1;
            e = (e | v) >> 1;
        }
        M::op(&l, &r)
    }

    /// 全体での総積を返す. 空のセグ木の場合はNoneを返す
    ///
    /// # Complexity
    ///
    /// * *O*(1)
    #[inline]
    pub fn all_prod(&self) -> Option<&M::T> {
        self.d.last()
    }
}

#[cfg(test)]
mod tests {
    use super::super::algebra::Add;
    use super::*;

    #[test]
    fn segment_tree_works() {
        let mut seg = SegmentTree::<Add<i32>>::from([
            3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4,
        ]);
        assert_eq!(*seg.get(3), 1);
        assert_eq!(seg.prod(6..12), 29);

        seg.set(10, 6);

        assert_eq!(seg.prod(6..12), 30);
        assert_eq!(seg.all_prod(), Some(&98));
    }

    #[test]
    fn case_empty() {
        let seg = SegmentTree::<Add<i32>>::from([]);

        assert_eq!(seg.len(), 0);
        assert_eq!(seg.prod(..), 0);
        assert_eq!(seg.all_prod(), None);
    }
}
