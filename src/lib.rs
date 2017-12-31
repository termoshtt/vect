#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
use core::ops::*;

#[cfg(not(feature = "no_std"))]
use std::ops::*;

extern crate num_traits;
use num_traits::Zero;

/// Trait based Vector
///
/// This trait hides how the memory is allocated.
/// We can write generic algorithm under an assumption that memory is
/// allocated continously since it can be derefed to `std::slice`.
/// This will be useful with custom allocators.
pub trait Vect<T: Clone >: Deref<Target = [T]> + DerefMut + Clone {
    fn zeros(usize) -> Self where T: Zero;
}

#[cfg(not(feature = "no_std"))]
impl<T: Clone + Zero> Vect<T> for Vec<T> {
    fn zeros(n: usize) -> Self {
        vec![T::zero(); n]
    }
}
