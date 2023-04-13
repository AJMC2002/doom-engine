use std::ops::Add;

#[derive(Clone, PartialEq, Eq, Debug, Default, PartialOrd, Ord)]
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector { data: Vec::new() }
    }
    pub fn from_vec(v: Vec<T>) -> Vector<T> {
        Vector { data: v }
    }
}

impl<T> Add<Vector<T>> for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn add(self, other: Vector<T>) -> Vector<T> {
        Vector {
            data: (0..self.data.len())
                .map(|i| match i < other.data.len() {
                    true => self.data[i] + other.data[i],
                    false => self.data[i],
                })
                .collect(),
        }
    }
}

impl<'a, T> Add<&'a Vector<T>> for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn add(self, other: &'a Vector<T>) -> Vector<T> {
        self + other.clone()
    }
}

impl<'a, T> Add<Vector<T>> for &'a Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn add(self, other: Vector<T>) -> Vector<T> {
        self.clone() + other
    }
}

impl<'a, 'b, T> Add<&'a Vector<T>> for &'b Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn add(self, other: &'a Vector<T>) -> Vector<T> {
        self.clone() + other.clone()
    }
}
