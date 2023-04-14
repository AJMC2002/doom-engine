#[cfg(test)]
mod tests {
    use doom_engine::maths::*;
    use doom_engine::vector;

    #[test]
    fn vector_sum() {
        let a = Vector::from_vec(vec![0.5, 0.2, 0.2]);
        let b = Vector::from_vec(vec![0.5, 0.0, 0.2]);
        assert_eq!(&a + b, vector![1.0, 0.2, 0.4]);
    }

    #[test]
    fn vector_mult() {
        let a = vector![0.5, 0.2, 0.2];
        assert_eq!(&a * &2.0, vector![1.0, 0.4, 0.4]);
    }

    #[test]
    fn dot_prod() {
        let a = Vector::from_vec(vec![0.5, 0.2, 0.2]);
        let b = Vector::from_vec(vec![0.5, 0.0, 0.2]);
        assert_eq!(a * b, 0.29);
    }
}
