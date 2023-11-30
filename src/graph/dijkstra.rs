use super::{
    super::radix_heap::{Radix, RadixHeap},
    AddEdgeWeight,
};

#[derive(Clone)]
pub struct Dijkstra<T> {
    d: Box<[Vec<(usize, T)>]>,
}

impl<T> Dijkstra<T> {
    #[must_use]
    pub fn new(n: usize) -> Dijkstra<T> {
        let mut d = Vec::with_capacity(n);
        for _ in 0..n {
            d.push(vec![]);
        }
        Dijkstra { d: d.into() }
    }
}

impl<T> AddEdgeWeight<T, true> for Dijkstra<T> {
    #[inline]
    fn add_edge(&mut self, from: usize, to: usize, weight: T) {
        debug_assert!(from < self.d.len());
        debug_assert!(to < self.d.len());

        self.d[from].push((to, weight));
    }
}

#[derive(Clone, Copy)]
struct Pair<T>(usize, T);
impl<T: PartialEq> PartialEq for Pair<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl<T: Eq> Eq for Pair<T> {}

impl<T: PartialOrd> PartialOrd for Pair<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl<T: Ord> Ord for Pair<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

impl<T: num::Bounded> num::Bounded for Pair<T> {
    #[inline]
    fn min_value() -> Self {
        Pair(usize::MAX, T::max_value())
    }

    #[inline]
    fn max_value() -> Self {
        Pair(usize::MAX, T::min_value())
    }
}

impl<T: Radix + num::Bounded> Radix for Pair<T> {
    #[inline]
    fn dist(&self, r: &Self) -> u32 {
        self.1.dist(&r.1)
    }

    const BITS: u32 = T::BITS;
}

impl<T: num::Zero + Radix + std::ops::Add<Output = T>> Dijkstra<T> {
    #[must_use]
    pub fn distance(&self, from: usize, to: usize) -> Option<T> {
        debug_assert!(from < self.d.len());
        debug_assert!(to < self.d.len());

        let mut k = RadixHeap::new();
        let mut d = vec![T::max_value(); self.d.len()].into_boxed_slice();
        k.push(Pair(from, T::zero()));
        d[from] = T::zero();
        while let Some(Pair(i, v)) = k.pop() {
            if i == to {
                return Some(v);
            }
            if v != d[i] {
                continue;
            }

            for &(j, c) in self.d[i].iter() {
                let y = c + v;
                if y < d[j] {
                    d[j] = y;
                    k.push(Pair(j, y));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dijkstra_works() {
        let mut d = Dijkstra::new(3);
        d.add_edge(1, 2, 3);

        assert_eq!(d.distance(0, 2), None);

        d.add_edge(0, 1, 3);
        d.add_edge(0, 2, 7);

        assert_eq!(d.distance(0, 2), Some(6));

        d.add_edge(0, 2, 5);

        assert_eq!(d.distance(0, 2), Some(5));
    }
}
