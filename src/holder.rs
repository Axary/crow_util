//! A module containing a basic holder struct, which is used for immutable access to its elements while still being able to insert new elements.
//!
//! For examples and further explanation, visit [`Holder<T>`](struct.Holder.html).
use std::cell::UnsafeCell;
use std::collections::HashMap;


/// A Hashmap which allows for immutable access while still allowing the addition of new objects.
///
/// I am still searching for a name which clearly expresses what this struct is doing, so the name might change in the future.
/// Functionality should already be stable.
///
/// # Examples
///
/// ```
/// use crow_util::holder;
///
/// let holder = holder::Holder::new();
/// holder.insert("a", 7);
/// holder.insert("b", 15);
/// holder.insert("c", 19);
///
/// assert_eq!(holder.get("a"), Some(&7));
/// 
/// {
///     let y = holder.insert("d", 54);
///
///     assert_eq!(holder.insert("d",84), Some(&54));
/// }
///
/// assert_eq!(holder.len(),4);
///
/// let mut holder = holder;
/// holder.clear();
/// assert_eq!(holder.len(),0);
/// holder.shrink_to_fit();
/// assert_eq!(holder.capacity(),0);
/// ```
pub struct Holder<T: ?Sized> {
    items: UnsafeCell<HashMap<String,Box<T>>>,
}

impl<T> Holder<T> {
    /// Constructs a new, empty `Holder<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crow_util::holder;
    ///
    /// let holder: holder::Holder<u32> = holder::Holder::new();
    /// assert_eq!(holder.len(),0);
    /// ```
    pub fn new() -> Self {
        Holder {
            items: UnsafeCell::new(HashMap::new()),
        }
    }

    /// Constructs a new, empty `Holder<T>` with the specified capacity.
    /// The map will be able to hold exactly `capacity` elements without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use crow_util::holder;
    ///
    /// let mut holder = holder::Holder::with_capacity(42);
    /// assert!(holder.capacity() >= 42);
    /// # holder.insert("hidden", 420);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Holder {
            items: UnsafeCell::new(HashMap::with_capacity(capacity)),
        }
    }

    /// Returns a reference to the element corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use crow_util::holder;
    ///
    /// let holder = holder::Holder::new();
    /// holder.insert("a", 42);
    /// assert_eq!(holder.get("a"), Some(&42));
    /// ```
    pub fn get(&self, key: &str) -> Option<&T> {
        let items = unsafe {& *self.items.get() };
        items.get(key).map(|v| &**v)
    }

    /// Inserts an `element` accessible by `key`.
    /// 
    /// In case the `key` was already present, the old `element` is returned and the new one is ignored.
    /// This method can be used while `Holder<T>` is already immutably borrowed.
    ///
    /// # Examples
    ///
    /// ```
    /// use crow_util::holder;
    ///
    /// let holder = holder::Holder::new();
    /// assert_eq!(holder.insert("a", 42), None);
    ///
    /// let val = holder.get("a");
    /// assert_eq!(holder.insert("a", 25), val);
    ///
    /// holder.insert("b",43);
    /// assert_eq!(holder.len(),2);
    /// ```
    pub fn insert(&self, key: &str, element: T) -> Option<&T> {
        let items = unsafe {&mut *self.items.get() };
        if items.contains_key(key) {
            items.get(key).map(|v| &**v)
        }
        else {
            assert!(items.insert(key.to_owned(), Box::new(element)).is_none());
            None
        }
    }

    /// Inserts an `element`, which is created by a closure and can be accessed by `key`,
    /// returning a usable reference `element` corresponding to this `key`.
    /// In case the `key` was already present, the old `element` is returned and the new one is ignored.
    /// This method can be used while `Holder<T>` is already immutably borrowed.
    ///
    /// This is useful in case performance is important, due to the fact that the closure is only called in
    /// case the `key` does not already exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use crow_util::holder;
    ///
    /// fn complex_calculation(x: f64, y: f64) -> f64 {
    /// #   1.0*x*y
    ///     // ... this function is really long and complex.
    /// }
    ///
    /// let holder = holder::Holder::new();
    ///
    /// // this calls complex_calculation() only once.
    /// for _ in 0..10_000 {
    ///     holder.insert_fn("a", || complex_calculation(2.0, 42.0));
    /// }
    ///
    /// // this calls complex_calculation() 10_000 times,
    /// for _ in 0..10_000 {
    ///     holder.insert("b", complex_calculation(2.0, 42.0));
    /// }
    /// ```
    pub fn insert_fn<F>(&self, key: &str, element: F) -> Option<&T>
    where F: Fn() -> T {
        let items = unsafe {&mut *self.items.get() };
        if items.contains_key(key) {
            items.get(key).map(|v| &**v)
        }
        else {
            assert!(items.insert(key.to_owned(), Box::new(element())).is_none());
            None
        }
    }

    /// Clears the map, removing all `key`-`element` pairs. Keeps the allocated memory for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use crow_util::holder;
    ///
    /// let mut holder = holder::Holder::new();
    /// holder.insert("a", 42);
    /// holder.insert("b", 360);
    /// holder.insert("c", 7);
    /// assert_eq!(holder.len(), 3);
    /// let prev_capacity = holder.capacity();
    ///
    /// holder.clear();
    /// assert_eq!(holder.capacity(), prev_capacity);
    /// ```
    #[inline(always)]
    pub fn clear(&mut self) {
        unsafe { &mut *self.items.get() }.clear();
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use crow_util::holder;
    ///
    /// let mut holder = holder::Holder::new();
    /// holder.insert("a", 42);
    /// holder.insert("b", 360);
    /// holder.insert("c", 7);
    /// assert_eq!(holder.len(), 3);
    /// ```
    #[inline(always)]
    pub fn len(&self) -> usize {
        unsafe { & *self.items.get() }.len()
    }

    /// Shrinks the capacity of the map as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use crow_util::holder;
    ///
    /// let mut holder = holder::Holder::new();
    /// holder.insert("a", 42);
    /// holder.clear();
    /// let prev_capacity = holder.capacity();
    ///
    /// holder.shrink_to_fit();
    /// assert_ne!(holder.capacity(), prev_capacity);
    /// ```
    #[inline(always)]
    pub fn shrink_to_fit(&mut self) {
        unsafe { &mut *self.items.get() }.shrink_to_fit();
    }

    /// Returns the number of elements the map can hold without reallocating.
    /// This number is a lower bound, meaning that the `Holder<T>` might be able to hold more, but is guaranteed to be able to hold at least this many.
    ///
    /// # Examples
    ///
    /// ```
    /// use crow_util::holder;
    ///
    /// let mut holder = holder::Holder::new();
    /// let capacity = holder.capacity();
    ///
    /// let mut key = "o".to_string();
    /// for i in 0..capacity {
    ///     key.push('o');
    ///     holder.insert(&key,i);
    /// }
    ///
    /// assert_eq!(capacity, holder.capacity());
    /// ```
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        unsafe { & *self.items.get() }.capacity()
    }
}
