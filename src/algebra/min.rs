use super::{Associative, Commutative, Idempotent, Identity, Magma};

pub struct Min<T> {
    _p: std::marker::PhantomData<fn() -> T>,
}
impl<T> Magma for Min<T>
where
    T: Clone + Ord,
{
    type T = T;

    #[inline]
    fn op(a: &T, b: &T) -> T {
        if a > b {
            b.clone()
        } else {
            a.clone()
        }
    }
}
impl<T> Associative for Min<T> where Min<T>: Magma<T = T> {}
impl<T> Commutative for Min<T> where Min<T>: Magma<T = T> {}
impl<T> Idempotent for Min<T> where Min<T>: Magma<T = T> {}
impl<T> Identity for Min<T>
where
    T: num::Bounded,
    Min<T>: Magma<T = T>,
{
    #[inline]
    fn e() -> T {
        T::max_value()
    }
}
