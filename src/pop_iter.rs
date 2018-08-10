//! A module containing a struct which implements `iter()` to pop a `Vec<T>` until it is empty.

/// An iterator over a `Vec<T>` which pops the last element each time `next()` is called.
///
/// This iterator manually implements many methods of the `Iterator` trait to be as fast as possible.
///
/// # Examples
///
/// ```
/// use crow_util::pop_iter::ToPopIter;
///
/// let mut vec = vec![1,2,3,4,5];
/// {
///     let pop_iter = vec.pop_iter();
///
///     for (i, item) in pop_iter.enumerate() {
///         assert_eq!(5-i, item);
///     }
/// }
/// assert!(vec.is_empty());
/// ```
pub struct PopIter<'a, T: 'a> {
    vec: &'a mut Vec<T>,
}

impl<'a, T> Iterator for PopIter<'a, T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.vec.pop()
    }

    fn count(self) -> usize {
        self.vec.len()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.vec.len()))
    }

    fn last(self) -> Option<Self::Item>  {
        if self.vec.is_empty() {
            None
        }
        else {
            Some(self.vec.swap_remove(0))
        }
    }
}

/// Conversion into a `PopIter`.
pub trait ToPopIter<T> {
    fn pop_iter(&mut self) -> PopIter<T>;
}

impl<T> ToPopIter<T> for Vec<T> {
    fn pop_iter(&mut self) -> PopIter<T> {
        PopIter {
            vec: self,
        }
    }
}
