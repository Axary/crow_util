//! This module contains traits which are needed by the [`crow_engine`].
//!
//! For more information and examples, look at each individual actual traits.
//! 
//! [`crow_engine`]:https://crates.io/crates/crow_engine

/// Used to remove all elements from a collection for which the predicate `f` does not return true.
pub trait RetainMut<T> {
    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` such that `f(&mut e)` returns `false`.
    /// This method operates in place and preserves the order of the retained
    /// elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use crow_util::traits::*;
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4, 5];
    /// vec.retain_mut(|x| { *x += 1; *x % 2 == 0 } );
    /// assert_eq!(vec, [2, 4, 6]);
    /// ```
    fn retain_mut<F>(&mut self, f: F)
        where F: FnMut(&mut T) -> bool;
}

impl<T> RetainMut<T> for Vec<T> {
    fn retain_mut<F>(&mut self, mut f: F)
        where F: FnMut(&mut T) -> bool
    {
            let len = self.len();
            let mut del = 0;
            {
                let v = &mut **self;

                for i in 0..len {
                    if !f(&mut v[i]) {
                        del += 1;
                    } else if del > 0 {
                    v.swap(i - del, i);
                }
            }
        }
        if del > 0 {
            self.truncate(len - del);
        }
    }
}

/// Used to mutably borrow 2 elements from a collection at once.
pub trait GetTwo<T> {
    /// Mutably borrows 2 elements at once.
    /// 
    /// In case `index_a` is equal to `index_b` or one of them is out of bounds this function returns `None`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crow_util::traits::*;
    /// 
    /// let mut x = vec![0, 1, 2, 3, 4, 5];
    /// assert_eq!(x.get_two(0, 3), Some((&mut 0, &mut 3)));
    /// assert_eq!(x.get_two(1, 1), None);
    /// ```
    fn get_two(&mut self, index_a: usize, index_b: usize) -> Option<(&mut T, &mut T)>;

    /// Mutably borrows 2 elements at once without checking if it is safe to do so.TakeTwo
    /// 
    /// This is genarally not recommended, use with caution! For a safe alternative see [`get_two`](#tymethod.get_two)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crow_util::traits::*;
    /// 
    /// let mut x = vec![0, 1, 2, 3, 4, 5];
    /// assert_eq!(unsafe { x.get_two_unchecked(0, 3) }, (&mut 0, &mut 3));
    /// ```
    unsafe fn get_two_unchecked(&mut self, index_a: usize, index_b: usize) -> (&mut T, &mut T);
}

impl<T> GetTwo<T> for Vec<T> {
    fn get_two(&mut self, index_a: usize, index_b: usize) -> Option<(&mut T, &mut T)> {
        if index_a != index_b && index_a < self.len() && index_b < self.len() {
            Some(unsafe { self.get_two_unchecked(index_a, index_b) })    
        }
        else {
            None
        }
    }

    unsafe fn get_two_unchecked(&mut self, index_a: usize, index_b: usize) -> (&mut T, &mut T) {
        let ar = &mut *(self.get_unchecked_mut(index_a) as *mut _);
        let br = self.get_unchecked_mut(index_b);
        (ar, br)
    }
}