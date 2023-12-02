/// # Constraints
///
/// * `f(a, b) && f(b, c)` ⇒ `f(a, c)`
/// * `!f(a, b) && !f(b, c)` ⇒ `!f(a, c)`
/// * `!f(a, b) && f(a, c)` ⇒ `f(b, c)`
///
/// # Complexity
///
/// * *Θ*(*n* log *n*)
pub fn lis<T, F: Fn(&T, &T) -> bool>(slice: &[T], f: F) -> Box<[usize]> {
    if slice.is_empty() {
        return [].into();
    }
    let mut d = vec![];
    let mut r = Vec::with_capacity(slice.len());
    for (i, v) in slice.iter().enumerate() {
        let mut ok = 0;
        let mut ng = d.len() + 1;
        while ng - ok > 1 {
            let mid = (ng + ok) >> 1;
            if f(&slice[d[mid - 1]], v) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        if ok == 0 {
            r.push(i);
        } else {
            r.push(d[ok - 1]);
        }
        if ok == d.len() {
            d.push(i);
        } else {
            d[ok] = i;
        }
    }

    let mut k = Vec::with_capacity(d.len());
    let mut g = *d.last().unwrap();
    k.push(g);
    while g != r[g] {
        g = r[g];
        k.push(g);
    }
    k.reverse();
    k.into()
}

/// # Constraints
///
/// * `f(a, b) && f(b, c)` ⇒ `f(a, c)`
/// * `!f(a, b) && !f(b, c)` ⇒ `!f(a, c)`
/// * `!f(a, b) && f(a, c)` ⇒ `f(b, c)`
///
/// # Complexity
///
/// * *Θ*(*n* log *n*)
pub fn lis_len<T, F: Fn(&T, &T) -> bool>(slice: &[T], f: F) -> usize {
    let mut d = vec![];
    for (i, v) in slice.iter().enumerate() {
        let mut ok = 0;
        let mut ng = d.len() + 1;
        while ng - ok > 1 {
            let mid = (ng + ok) >> 1;
            if f(&slice[d[mid - 1]], v) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        if ok == d.len() {
            d.push(i);
        } else {
            d[ok] = i;
        }
    }

    d.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lis_works() {
        assert_eq!(lis_len(&[1, 2, 3, 3, 4, 5, 2], |a, b| a < b), 5);
        assert_eq!(lis_len(&[1, 2, 3, 3, 4, 5, 2], |a, b| a <= b), 6);
        assert_eq!(lis_len(&[1, 2, 3, 3, 4, 5, 2], |a, b| a > b), 2);
        assert_eq!(lis_len(&[1, 2, 3, 3, 4, 5, 2], |a, b| a >= b), 3);

        assert_eq!(
            lis(&[1, 2, 3, 3, 4, 5, 2], |a, b| a <= b),
            [0, 1, 2, 3, 4, 5].into()
        );
        assert_eq!(lis(&[1, 2, 3, 3, 4, 5, 2], |a, b| a >= b), [2, 3, 6].into());
    }

    #[test]
    fn small_case() {
        assert_eq!(lis(&[], |a: &i32, b| a < b), [].into());
        assert_eq!(lis_len(&[], |a: &i32, b| a < b), 0);

        assert_eq!(lis(&[5], |a: &i32, b| a < b), [0].into());
        assert_eq!(lis_len(&[5], |a: &i32, b| a < b), 1);
    }
}
