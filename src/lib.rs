#![doc = include_str!("../README.md")]

/// An extension trait for `Vec<T>` that provides a [`PushMut::push_mut`] method.
pub trait PushMut<T> {
    /// Pushes a value to the back of the vector, and returns a mutable reference to it.
    fn push_mut(&mut self, value: T) -> &mut T;
}

impl<T> PushMut<T> for Vec<T> {
    fn push_mut(&mut self, value: T) -> &mut T {
        self.push(value);
        unsafe { self.last_mut().unwrap_unchecked() }
    }
}
