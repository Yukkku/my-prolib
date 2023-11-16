use super::{Associative, Commutative, Idempotent, Identity, Magma};

pub struct Max<T> {
    _p: std::marker::PhantomData<fn() -> T>,
}
impl<T> Magma for Max<T>
where
    T: Clone + Ord,
{
    type T = T;

    #[inline]
    fn op(a: &T, b: &T) -> T {
        if a > b {
            a.clone()
        } else {
            b.clone()
        }
    }
}
impl<T> Associative for Max<T> where Max<T>: Magma<T = T> {}
impl<T> Commutative for Max<T> where Max<T>: Magma<T = T> {}
impl<T> Idempotent for Max<T> where Max<T>: Magma<T = T> {}
impl<T> Identity for Max<T>
where
    T: num::Bounded,
    Max<T>: Magma<T = T>,
{
    #[inline]
    fn e() -> T {
        T::min_value()
    }
}
