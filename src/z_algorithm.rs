#[must_use]
pub fn z_algorithm<T: PartialEq>(v: &[T]) -> Box<[usize]> {
    if v.is_empty() {
        return [].into();
    }
    if v.len() == 1 {
        return [1].into();
    }

    let mut l = 0;
    while l + 1 != v.len() && v[l] == v[l + 1] {
        l += 1;
    }

    let mut r = Vec::with_capacity(v.len());
    r.push(v.len());
    r.push(l);

    let mut j = 1;
    for i in 2..v.len() {
        let k = r[i - j];
        if k + (i - j) < l {
            r.push(k);
            continue;
        }
        l = (l + j).checked_sub(i).unwrap_or(0);
        while i + l != v.len() && v[l] == v[i + l] {
            l += 1;
        }
        r.push(l);
        j = i;
    }

    r[0] = v.len();
    r.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn z_algorithm_works() {
        assert_eq!(
            z_algorithm(b"abracadabra"),
            [11, 0, 0, 1, 0, 1, 0, 4, 0, 0, 1].into()
        );
        assert_eq!(
            z_algorithm(b"niningashi"),
            [10, 0, 3, 0, 1, 0, 0, 0, 0, 0].into()
        );
        assert_eq!(
            z_algorithm(b"aaaaaaaaaa"),
            [10, 9, 8, 7, 6, 5, 4, 3, 2, 1].into()
        );
    }

    #[test]
    fn small_case() {
        assert_eq!(z_algorithm(b""), [].into());
        assert_eq!(z_algorithm(b"A"), [1].into());
        assert_eq!(z_algorithm(b"AA"), [2, 1].into());
        assert_eq!(z_algorithm(b"AB"), [2, 0].into());
    }
}
