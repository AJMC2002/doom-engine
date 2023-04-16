#[cfg(test)]
mod tests {
    use std::vec;

    use doom_engine::maths::*;
    use doom_engine::vector;

    #[test]
    fn make_vector() {
        let a = vector![1., 2., 3.];
        assert_eq!(a.len(), 3);
        assert_eq!(a.abs(), 14_f32.sqrt());
        assert_eq!(a[0], 1.);
        assert_eq!(a[1], 2.);
        assert_eq!(a[2], 3.);
    }

    #[test]
    fn make_vector_from_vec() {
        let a = Vector::from_vec(vec![1., 2., 3.]);
        let b = vector![1., 2., 3.];
        assert_eq!(a, b);
    }

    #[test]
    fn make_vector_from_iter() {
        let a = Vector::from_iter(vec![1., 2., 3.]);
        let b = Vector::from_iter((1..4).map(|x| x as f32));
        let c = vector![1., 2., 3.];
        assert_eq!(a, c);
        assert_eq!(b, c);
    }

    #[test]
    fn scalar_ops() {
        let a = vector![1., 2., 3.];
        assert_eq!(&a + 2, vector![3., 4., 5.]);
        assert_eq!(&a - 1, vector![0., 1., 2.]);
        assert_eq!(&a * 3, vector![3., 6., 9.]);
        assert_eq!(&a / 2, vector![0.5, 1., 1.5]);
    }

    #[test]
    fn vector_ops() {
        let a = vector![1., 2., 3.];
        let b = vector![1., 2., 3.];
        assert_eq!(&a + &b, vector![2., 4., 6.]);
        assert_eq!(&a - &b, vector![0., 0., 0.]);
        assert_eq!(&a * &b, 14.);
        assert_eq!(a.cross(&b), vector![12., 6., 4.]);
    }

    #[test]
    fn scalar_assignment_ops() {
        let mut a = vector![1., 2., 3.];
        a += 2;
        assert_eq!(a, vector![3., 4., 5.]);
        a -= 1;
        assert_eq!(a, vector![2., 3., 4.]);
        a *= 3;
        assert_eq!(a, vector![6., 9., 12.]);
        a /= 2;
        assert_eq!(a, vector![3., 4.5, 6.]);
    }

    #[test]
    fn vector_assignment_ops() {
        let mut a = vector![1., 2., 3.];
        a += vector![1., 2., 3.];
        assert_eq!(a, vector![2., 4., 6.]);
        a -= vector![1., 2., 3.];
        assert_eq!(a, vector![1., 2., 3.]);
    }
}
