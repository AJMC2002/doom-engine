#[cfg(test)]
mod tests {
    mod vector_tests {
        use doom_engine::maths::*;
        use doom_engine::vector;
        use std::vec;

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

    mod matrix_tests {
        use doom_engine::{
            maths::{Matrix, Vector},
            matrix, vector,
        };

        #[test]
        fn new_matrix() {
            let mut m = Matrix::zeroes(3, 3);
            assert_eq!(m.rows(), 3);
            assert_eq!(m.cols(), 3);
            assert_eq!(m[0], [0., 0., 0.]);
            assert_eq!(m[0][0], 0.);
            m[1][1] = 25.;
            assert_eq!(m[1], [0., 25., 0.]);
            assert_eq!(m[1][1], 25.);
        }

        #[test]
        fn from_constructors() {
            let m1 = Matrix::from_slice(&[
                &[1., 2., 3., 4.],
                &[1., 2., 3., 4.],
                &[1., 2., 3., 4.],
                &[1., 2., 3., 4.],
            ]);
            assert_eq!(m1.rows(), 4);
            assert_eq!(m1.cols(), 4);
            assert_eq!(m1[0][0], 1.);
            assert_eq!(m1[3][3], 4.);

            let m2 = Matrix::from_vec(vec![vec![1., 2.], vec![1., 2.]]);
            assert_eq!(m2.rows(), 2);
            assert_eq!(m2.cols(), 2);
            assert_eq!(m2[0][0], 1.);
            assert_eq!(m2[1][1], 2.);
        }

        #[test]
        fn matrix_macro() {
            let m = matrix![[1., 2., 3.], [1., 2., 3.]];
            assert_eq!(m.rows(), 2);
            assert_eq!(m.cols(), 3);
            assert_eq!(m[0], [1., 2., 3.]);
            assert_eq!(m[0][0], 1.);
        }

        #[test]
        fn determinant() {
            let m1 = matrix![[2.]];
            assert_eq!(m1.det(), 2.);
            let m2 = matrix![[2., -1.], [-1., 2.]];
            assert_eq!(m2.det(), 3.);
            let m3 = matrix![[2., -1., 0.], [-1., 2., -1.], [0., -1., 2.]];
            assert_eq!(m3.det(), 4.);
        }

        #[test]
        fn mul_with_vec() {
            let m = matrix![[2., 43., 243.], [3., 123., 11.], [4., 10., 100.]];
            let v = vector![1., 2., 3.];
            let result = matrix![
                [2. + 43. * 2. + 243. * 3.],
                [3. + 123. * 2. + 11. * 3.],
                [4. + 10. * 2. + 100. * 3.]
            ];
            assert_eq!(m * v, result);
        }

        #[test]
        fn mul_with_mat() {
            let m1 = matrix![[1., 4., 7.], [2., 5., 8.], [3., 6., 9.]];
            let m2 = matrix![[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]];
            let result12 = matrix![[66., 78., 90.], [78., 93., 108.], [90., 108., 126.]];
            assert_eq!(&m1 * &m2, result12);
            let result21 = matrix![[14., 32., 50.], [32., 77., 122.], [50., 122., 194.]];
            assert_eq!(&m2 * &m1, result21);
        }
    }
}
