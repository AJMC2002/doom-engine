use crate::maths::Matrix;

pub struct Context {
    pub projection: Matrix,
    pub view: Matrix,
}

impl Context {
    pub fn new(projection: Matrix, view: Matrix) -> Self {
        Context { projection, view }
    }
}
