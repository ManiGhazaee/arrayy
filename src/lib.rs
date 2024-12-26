use std::{
    fmt::Debug,
    mem,
    ops::{Deref, DerefMut, Range},
    ptr,
};

mod tests;

#[derive(Clone, Copy)]
pub struct Array<T, const L: usize> {
    data: [T; L],
    len: usize,
}

impl<T: Default + Copy, const L: usize> Default for Array<T, L> {
    fn default() -> Self {
        Self {
            data: [T::default(); L],
            len: 0,
        }
    }
}

impl<T: Copy + Default, const L: usize> Array<T, L> {
    /// Returns the number of elements in the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let arr = array!(1, 2, 3);
    /// assert_eq!(arr.len(), 3);
    /// ```
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Sets the length of the array.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `len` is less than or equal to the capacity of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let mut arr = array!(i32; 5);
    /// unsafe { arr.set_len(3) };
    /// assert_eq!(arr.len(), 3);
    /// ```
    pub unsafe fn set_len(&mut self, len: usize) {
        if len > L {
            panic!("len ({}) > capacity ({})", len, L);
        }
        self.len = len;
    }

    /// Returns a reference to the underlying buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let arr = array!(1, 2, 3; 5);
    /// unsafe { assert_eq!(arr.buf(), &[1, 2, 3, 0, 0]) };
    /// ```
    pub const fn buf(&self) -> &[T; L] {
        &self.data
    }

    /// Returns a mutable reference to the underlying buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let mut arr = array!(i32; 5);
    /// unsafe { arr.buf_mut()[0] = 1 };
    /// assert_eq!(unsafe { arr.buf() }, &[1, 0, 0, 0, 0]);
    /// ```
    pub fn buf_mut(&mut self) -> &mut [T; L] {
        &mut self.data
    }

    /// Returns the capacity of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let arr = array!(i32; 5);
    /// assert_eq!(arr.capacity(), 5);
    /// ```
    pub const fn capacity(&self) -> usize {
        L
    }

    /// Returns `true` if the array contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let arr = array!(i32; 5);
    /// assert!(arr.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns a reference to the element at the specified index, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let arr = array!(1, 2, 3);
    /// assert_eq!(arr.get(1), Some(&2));
    /// assert_eq!(arr.get(3), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.data.get_unchecked(index) })
        }
    }

    /// Returns a mutable reference to the element at the specified index, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let mut arr = array!(1, 2, 3);
    /// if let Some(x) = arr.get_mut(1) {
    ///     *x = 4;
    /// }
    /// assert_eq!(arr.get(1), Some(&4));
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.data.get_unchecked_mut(index) })
        }
    }

    /// Returns a reference to the element at the specified index without bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is within bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let arr = array!(1, 2, 3);
    /// unsafe { assert_eq!(arr.get_unchecked(1), &2) };
    /// ```
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.data.get_unchecked(index)
    }

    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.data.get_unchecked_mut(index)
    }

    pub fn last(&self) -> Option<&T> {
        if self.len > 0 {
            Some(unsafe { self.get_unchecked(self.len - 1) })
        } else {
            None
        }
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.len > 0 {
            Some(unsafe { self.get_unchecked_mut(self.len - 1) })
        } else {
            None
        }
    }

    pub fn first(&self) -> Option<&T> {
        self.get(0)
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    pub fn from(arr: &[T]) -> Self {
        let mut s = Self {
            data: [T::default(); L],
            len: arr.len(),
        };
        s.data[0..arr.len()].copy_from_slice(arr);
        s
    }

    pub fn push(&mut self, val: T) {
        self.data[self.len] = val;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        Some(mem::take(self.data.get_mut(self.len).unwrap()))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data[0..self.len].iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data[0..self.len].iter_mut()
    }

    /// Appends the elements of another array to the end of this array.
    ///
    /// # Panics
    ///
    /// Panics if the combined length of both arrays exceeds the capacity of this array.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let mut arr1 = array!(1, 2, 3; 5);
    /// let arr2 = array!(4, 5);
    /// arr1.append(&arr2);
    /// assert_eq!(arr1, array!(1, 2, 3, 4, 5));
    /// ```
    pub fn append<const M: usize>(&mut self, other: &Array<T, M>) {
        if other.len > L - self.len {
            panic!()
        }
        self.data[self.len..self.len + other.len].copy_from_slice(&other.data[0..other.len]);
        self.len += other.len;
    }

    /// Filters the elements of the array, returning a new array with only the elements that match the predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let arr = array!(1, 2, 3, 4, 5);
    /// let filtered = arr.filter(|&x| x % 2 == 0);
    /// assert_eq!(filtered, array!(2, 4));
    /// ```
    pub fn filter(mut self, mut predicate: impl FnMut(&T) -> bool) -> Self {
        let mut j = 0;
        for i in 0..self.len {
            unsafe {
                let elt = *self.data.get_unchecked(i);
                if predicate(&elt) {
                    *self.data.get_unchecked_mut(j) = elt;
                    j += 1;
                }
            }
        }
        self.len = j;
        self
    }

    /// Applies a function to each element in the array, returning a new array with the results.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let arr = array!(1, 2, 3);
    /// let mapped = arr.map(|&x| x * 2);
    /// assert_eq!(mapped, array!(2, 4, 6));
    /// ```
    pub fn map<X: Copy + Default>(self, mut f: impl FnMut(&T) -> X) -> Array<X, L> {
        let mut copy = Array::<X, L>::default();
        for i in 0..self.len {
            unsafe {
                let elt = *self.data.get_unchecked(i);
                *copy.get_unchecked_mut(i) = f(&elt);
            }
        }
        copy.len = self.len;
        copy
    }

    /// Truncates the array, keeping only the first `len` elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let mut arr = array!(1, 2, 3, 4, 5);
    /// arr.truncate(3);
    /// assert_eq!(arr, array!(1, 2, 3));
    /// ```
    pub fn truncate(&mut self, len: usize) {
        if len > self.len {
            return;
        }
        self.len = len;
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data[0..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data[0..self.len]
    }

    pub fn as_vec(&self) -> Vec<T> {
        self.as_slice().to_vec()
    }

    /// Inserts an element at the specified index, shifting all elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if the index is greater than the length of the array or if the array is at full capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let mut arr = array!(1, 2, 3; 5);
    /// arr.insert(1, 4);
    /// assert_eq!(arr, array!(1, 4, 2, 3));
    /// ```
    pub fn insert(&mut self, index: usize, element: T) {
        if index > self.len {
            panic!("insert index ({}) > len ({})", index, self.len);
        }
        if index == L {
            panic!("insert index ({}) == capacity ({})", index, L);
        }
        if self.len == L {
            panic!("len ({}) == capacity ({})", self.len, L);
        }
        unsafe {
            {
                let p = self.data.as_mut_ptr().add(index);
                if index < self.len {
                    ptr::copy(p, p.add(1), self.len - index);
                }
                ptr::write(p, element);
            }
            self.len += 1;
        }
    }

    /// Removes and returns the element at the specified index, shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let mut arr = array!(1, 2, 3);
    /// assert_eq!(arr.remove(1), 2);
    /// assert_eq!(arr, array!(1, 3));
    /// ```
    pub fn remove(&mut self, index: usize) -> T {
        let len = self.len;
        if index >= len {
            panic!("remove index ({}) >= len ({})", index, len);
        }
        unsafe {
            let ret;
            {
                let ptr = self.as_mut_ptr().add(index);
                ret = ptr::read(ptr);
                ptr::copy(ptr.add(1), ptr, len - index - 1);
            }
            self.len -= 1;
            ret
        }
    }

    /// Clears the array, removing all elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use arrayy::array;
    /// let mut arr = array!(1, 2, 3);
    /// arr.clear();
    /// assert_eq!(arr.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr_range(&mut self) -> Range<*mut T> {
        self.data.as_mut_ptr_range()
    }

    pub fn as_ptr_range(&self) -> Range<*const T> {
        self.data.as_ptr_range()
    }

    /// # Safety
    ///
    /// The caller must ensure that the `Array` (`self`) outlives the `UnsafeIter`'s pointer (`*const T`) to the `self.data`'s buffer.
    /// Failing to do so will result in undefined behavior.
    ///
    /// # Example of Undefined Behavior
    ///
    /// ```rust,no_run
    /// use arrayy::*;
    /// fn x() -> UnsafeIter<u8> {
    ///     let arr = Array::<u8, 10>::from(&[1, 2, 3]);
    ///     unsafe { arr.into_iter() }
    /// }
    /// let mut iter = x();
    /// println!("{}", iter.next().unwrap()); // can be anything
    /// ```
    pub unsafe fn into_iter(&self) -> UnsafeIter<T> {
        UnsafeIter {
            data: self.data.as_ptr(),
            idx: 0,
            len: self.len,
        }
    }
}

pub struct UnsafeIter<T> {
    data: *const T,
    idx: usize,
    len: usize,
}

impl<T: Copy> Iterator for UnsafeIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            let ret = unsafe { Some(*self.data.add(self.idx)) };
            self.idx += 1;
            ret
        } else {
            None
        }
    }
}

impl<T: Debug + Copy + Default, const L: usize> Debug for Array<T, L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_slice())
    }
}

impl<T: Copy + Default + PartialEq, const L: usize> Eq for Array<T, L> {}

impl<T: Copy + Default + PartialEq, const L: usize, const X: usize> PartialEq<Array<T, X>>
    for Array<T, L>
{
    fn eq(&self, other: &Array<T, X>) -> bool {
        self.len == other.len && self.as_slice() == other.as_slice()
    }
}

impl<T, const L: usize> AsRef<Array<T, L>> for Array<T, L> {
    fn as_ref(&self) -> &Array<T, L> {
        self
    }
}

impl<T: Copy + Default, const L: usize> AsRef<[T]> for Array<T, L> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const L: usize> AsMut<Array<T, L>> for Array<T, L> {
    fn as_mut(&mut self) -> &mut Array<T, L> {
        self
    }
}

impl<T: Copy + Default, const L: usize> AsMut<[T]> for Array<T, L> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: Copy + Default, const L: usize> Deref for Array<T, L> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: Copy + Default, const L: usize> DerefMut for Array<T, L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

#[macro_export]
macro_rules! count {
    ($e:expr) => {
        1
    };
    ($e:expr, $($es:expr),+) => {
        1 + $crate::count!($($es),+)
    };
}

#[macro_export]
macro_rules! array {
    () => {
        $crate::Array::default()
    };
    ($ty:ty ;$cap:expr) => {
        $crate::Array::<$ty, $cap>::default()
    };
    (;$cap:expr) => {
        $crate::Array::<_, $cap>::default()
    };
    ($($es:expr),+; $cap:expr) => {
        $crate::Array::<_, $cap>::from(&[$($es),+])
    };
    ($($es:expr),+) => {
        $crate::Array::<_, { $crate::count!($($es),+) }>::from(&[$($es),+])
    };
}
