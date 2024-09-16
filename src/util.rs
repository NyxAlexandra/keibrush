use std::fmt;

/// Implements [`std::fmt::Debug`] using an implementation of
/// [`std::fmt::Display`].
pub(crate) struct DisplayDebug<'a, T: ?Sized>(pub &'a T);

impl<T: fmt::Display + ?Sized> fmt::Debug for DisplayDebug<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
