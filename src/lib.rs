
extern crate num_traits;

use std::ops::*;
use num_traits::Zero;

pub trait Vect<T: Clone >: Deref<Target = [T]> + Clone {
    fn zeros(usize) -> Self where T: Zero;
}

impl<T: Clone + Zero> Vect<T> for Vec<T> {
    fn zeros(n: usize) -> Self {
        vec![T::zero(); n]
    }
}
