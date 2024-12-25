use std::{fmt::Debug, mem, ops::Range, ptr};

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
    pub const fn len(&self) -> usize {
        self.len
    }

    pub unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    pub unsafe fn buf(&self) -> &[T; L] {
        &self.data
    }

    pub unsafe fn buf_mut(&mut self) -> &mut [T; L] {
        &mut self.data
    }

    pub const fn capacity(&self) -> usize {
        L
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.data.get_unchecked(index) })
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.data.get_unchecked_mut(index) })
        }
    }

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

    pub fn append<const M: usize>(&mut self, other: &Array<T, M>) {
        if other.len > L - self.len {
            panic!()
        }
        self.data[self.len..self.len + other.len].copy_from_slice(&other.data[0..other.len]);
        self.len += other.len;
    }

    pub fn filter(self, mut predicate: impl FnMut(&T) -> bool) -> Self {
        let mut copy = self;
        let mut i = 0;
        let mut j = 0;
        while i < self.len {
            unsafe {
                let elt = *self.data.get_unchecked(i);
                if predicate(&elt) {
                    *copy.data.get_unchecked_mut(j) = elt;
                    j += 1;
                }
            }
            i += 1;
        }
        copy.len = j;
        copy
    }

    pub fn map<X: Copy + Default>(self, mut f: impl FnMut(&T) -> X) -> Array<X, L> {
        let mut copy = Array::<X, L>::default();
        let mut i = 0;
        while i < self.len {
            unsafe {
                let elt = *self.data.get_unchecked(i);
                *copy.get_unchecked_mut(i) = f(&elt);
            }
            i += 1;
        }
        copy.len = i;
        copy
    }

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

    pub fn insert(&mut self, index: usize, element: T) {
        if index > self.len {
            panic!("insert index ({}) > len ({})", index, self.len);
        }
        if index == L {
            panic!("insert index ({}) == capacity ({})", index, L);
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
