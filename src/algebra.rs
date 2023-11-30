pub trait Magma {
    type T;

    #[must_use]
    fn op(a: &Self::T, b: &Self::T) -> Self::T;
}

/// 要素`a`, `b`, `c`について`op(op(a, b), c)` = `op(a, op(b, c))`である
pub trait Associative: Magma {}
/// 要素`a`, `b`について`op(a, b)` = `op(b, a)`である
pub trait Commutative: Magma {}
/// 全ての`a`について`op(a, a)` = `a`である
pub trait Idempotent: Magma {}

/// 全ての`a`について`op(a, e)` = `op(e, a)` = `a`を満たす`e`が存在する
pub trait Identity: Magma {
    fn e() -> Self::T;
}

/// ある要素`a`について`op(a, b)` = `e`を満たす`b`が存在する
pub trait Inverse: Identity {
    #[must_use]
    fn rev(a: &Self::T) -> Self::T;
}

/// 要素`a`, `b`について`op(a, x)` = `b`を満たす`x`が一意である
pub trait Divisibility: Magma {
    #[must_use]
    fn div(a: &Self::T, b: &Self::T) -> Self::T;
}

mod add;
pub use add::Add;

mod min;
pub use min::Min;

mod max;
pub use max::Max;
