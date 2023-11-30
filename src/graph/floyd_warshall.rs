use super::AddEdgeWeight;

#[derive(Clone)]
pub struct FloydWarshall<T> {
    d: Box<[Option<T>]>,
    n: usize,
}

impl<T: num::Zero> FloydWarshall<T> {
    pub fn new(n: usize) -> Self {
        let mut d = Vec::with_capacity(n * 2);
        for i in 0..n {
            for j in 0..n {
                d.push(if i == j { Some(T::zero()) } else { None });
            }
        }
        FloydWarshall { d: d.into(), n }
    }
}
impl<T: Ord> FloydWarshall<T>
where
    for<'a> &'a T: std::ops::Add<Output = T>,
{
    #[must_use]
    pub fn apply(&mut self) -> FloydWarshallResult<T> {
        for i in 0..self.n {
            for j in 0..self.n {
                for k in 0..self.n {
                    let (Some(a), Some(b)) = (&self.d[j * self.n + i], &self.d[i * self.n + k])
                    else {
                        continue;
                    };
                    let y = a + b;
                    let v = &mut self.d[j * self.n + k];
                    if let Some(e) = v {
                        if y < *e {
                            *e = y;
                        }
                    } else {
                        *v = Some(y);
                    }
                }
            }
        }
        FloydWarshallResult { v: self }
    }
}

impl<T: Ord> AddEdgeWeight<T, true> for FloydWarshall<T> {
    fn add_edge(&mut self, from: usize, to: usize, weight: T) {
        debug_assert!(from < self.n);
        debug_assert!(to < self.n);

        let y = &mut self.d[from * self.n + to];
        if let Some(y) = y {
            if weight < *y {
                *y = weight;
            }
        } else {
            *y = Some(weight);
        }
    }
}

#[derive(Clone)]
pub struct FloydWarshallResult<'a, T> {
    v: &'a FloydWarshall<T>,
}
impl<'a, T> std::ops::Index<usize> for FloydWarshallResult<'a, T> {
    type Output = [Option<T>];

    fn index(&self, index: usize) -> &[Option<T>] {
        debug_assert!(index < self.v.n);

        &self.v.d[(self.v.n * index)..(self.v.n * (index + 1))]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floyd_warshall_works() {
        let mut v = FloydWarshall::new(4);
        v.add_edge(0, 1, 5);
        v.add_edge(0, 2, -1);
        v.add_edge(1, 3, 3);
        v.add_edge(2, 3, 1);
        v.add_edge(3, 2, 4);

        let r = v.apply();
        assert_eq!(r[0][1], Some(5));
        assert_eq!(r[1][0], None);
        assert_eq!(r[0][3], Some(0));
        assert_eq!(r[0][2], Some(-1));
    }
}
