use super::{Associative, Commutative, Divisibility, Identity, Inverse, Magma};

pub struct Add<T> {
    _p: std::marker::PhantomData<fn() -> T>,
}
impl<T> Magma for Add<T>
where
    for<'a> &'a T: std::ops::Add<Output = T>,
{
    type T = T;

    #[inline]
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        a + b
    }
}
impl<T> Associative for Add<T> where Add<T>: Magma<T = T> {}
impl<T> Commutative for Add<T> where Add<T>: Magma<T = T> {}
impl<T: num::Zero> Identity for Add<T>
where
    Add<T>: Magma<T = T>,
{
    #[inline]
    fn e() -> Self::T {
        Self::T::zero()
    }
}
impl<T> Inverse for Add<T>
where
    Add<T>: Identity<T = T>,
    for<'a> &'a T: std::ops::Neg<Output = T>,
{
    #[inline]
    fn rev(a: &Self::T) -> Self::T {
        -a
    }
}
impl<T> Divisibility for Add<T>
where
    Add<T>: Magma<T = T>,
    for<'a> &'a T: std::ops::Sub<Output = T>,
{
    #[inline]
    fn div(a: &Self::T, b: &Self::T) -> Self::T {
        a - b
    }
}
