use std::{mem::MaybeUninit, rc::Rc};
#[derive(Clone, Debug)]
enum Node<T> {
    Reaf(T),
    Root(Rc<Self>, Rc<Self>),
}
use Node::{Reaf, Root};

#[derive(Clone)]
pub struct PersistentArray<T>(Option<Node<T>>);

impl<T> PersistentArray<T> {
    pub fn new(v: Box<[T]>) -> Self {
        if v.is_empty() {
            return Self(None);
        }
        if v.len() == 1 {
            return Self(Some(Reaf(v.into_vec().into_iter().next().unwrap())));
        }
        let v = unsafe { std::mem::transmute::<_, Box<[MaybeUninit<T>]>>(v) };
        let mut p = v.len().next_power_of_two() >> 1;
        let mut qv = Vec::with_capacity(p);
        for i in 0..p {
            qv.push(if i | p < v.len() {
                Root(
                    Rc::new(Reaf(unsafe { v[i].assume_init_read() })),
                    Rc::new(Reaf(unsafe { v[i | p].assume_init_read() })),
                )
            } else {
                Reaf(unsafe { v[i].assume_init_read() })
            });
        }
        while p != 1 {
            let v = unsafe {
                std::mem::transmute::<_, Box<[MaybeUninit<Node<T>>]>>(qv.into_boxed_slice())
            };
            p >>= 1;
            let mut nq = Vec::with_capacity(p);
            for i in 0..p {
                nq.push(Root(
                    Rc::new(unsafe { v[i].assume_init_read() }),
                    Rc::new(unsafe { v[i | p].assume_init_read() }),
                ));
            }
            qv = nq;
        }
        Self(Some(unsafe { qv.pop().unwrap_unchecked() }))
    }

    pub fn set(&self, index: usize, value: T) -> Self {
        let Some(e) = &self.0 else {
            panic!();
        };
        let mut e = e;
        let mut w = vec![];
        let mut i = index;
        let mut j = 1;
        loop {
            match e {
                Reaf(_) => {
                    if i == 0 {
                        break;
                    } else {
                        panic!();
                    }
                }
                Root(l, r) => {
                    if i & 1 == 0 {
                        w.push(r);
                        e = l;
                    } else {
                        w.push(l);
                        e = r;
                    }
                }
            }
            i >>= 1;
            j <<= 1;
        }
        let mut n = Reaf(value);
        while let Some(r) = w.pop() {
            j >>= 1;
            if index & j == 0 {
                n = Root(Rc::new(n), r.clone());
            } else {
                n = Root(r.clone(), Rc::new(n));
            }
        }
        Self(Some(n))
    }

    pub fn get(&self, mut index: usize) -> Option<&T> {
        let Some(e) = &self.0 else {
            return None;
        };
        let mut e = e;
        loop {
            match e {
                Reaf(v) => {
                    if index == 0 {
                        return Some(&v);
                    } else {
                        return None;
                    }
                }
                Root(l, r) => {
                    if index & 1 == 0 {
                        e = l;
                    } else {
                        e = r;
                    }
                }
            }
            index >>= 1;
        }
    }
}

impl<T> std::ops::Index<usize> for PersistentArray<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persistent_array_works() {
        let r = PersistentArray::new([3, 1, 4, 1, 5, 9, 2].into());
        assert_eq!(r.get(0), Some(&3));
        assert_eq!(r.get(1), Some(&1));
        assert_eq!(r.get(2), Some(&4));
        assert_eq!(r.get(3), Some(&1));
        assert_eq!(r.get(4), Some(&5));
        assert_eq!(r.get(5), Some(&9));
        assert_eq!(r.get(6), Some(&2));
        assert_eq!(r.get(7), None);

        let r2 = r.set(4, 100);
        assert_eq!(r2.get(3), Some(&1));
        assert_eq!(r2.get(4), Some(&100));
        assert_eq!(r2.get(5), Some(&9));

        let r3 = r2.set(5, 150);
        assert_eq!(r3.get(3), Some(&1));
        assert_eq!(r3.get(4), Some(&100));
        assert_eq!(r3.get(5), Some(&150));
        assert_eq!(r3.get(6), Some(&2));

        let r4 = r2.set(3, 200);
        assert_eq!(r4.get(2), Some(&4));
        assert_eq!(r4.get(3), Some(&200));
        assert_eq!(r4.get(4), Some(&100));
        assert_eq!(r4.get(5), Some(&9));
    }
}
