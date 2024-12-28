#![cfg(test)]

use super::*;
#[test]
fn test_general() {
    let mut arr = Array::<usize, 5>::default();
    assert_eq!(arr.len, 0);
    arr.push(1);
    arr.push(2);
    arr.push(3);
    assert_eq!(arr.len, 3);
    let clone = arr.clone();
    let mut iter = clone.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);

    assert_eq!(arr.pop(), Some(3));
    assert_eq!(arr.pop(), Some(2));
    assert_eq!(arr.len, 1);
    assert_eq!(arr.pop(), Some(1));
    assert_eq!(arr.len, 0);
    assert_eq!(arr.pop(), None);
    assert_eq!(arr.len, 0);

    let mut arr1: Array<i32, 5> = Array::from(&[1, 2, 3]);
    assert_eq!(arr1.len, 3);
    let arr2: Array<i32, 5> = Array::from(&[4, 5]);
    assert_eq!(arr2.len, 2);

    arr1.append(&arr2);
    assert_eq!(arr1.len, 5);
    assert_eq!(arr2.len, 2);
    assert_eq!(arr1.data, [1, 2, 3, 4, 5]);

    let arr: Array<u8, 10> = Array::from(&[1, 2, 3]);
    unsafe {
        let mut unsafe_iter = arr.into_iter();
        assert_eq!(Some(1), unsafe_iter.next());
        assert_eq!(Some(2), unsafe_iter.next());
        assert_eq!(Some(3), unsafe_iter.next());
        assert_eq!(None, unsafe_iter.next());
    }
}

#[test]
fn test_new() {
    let arr1 = Array::new([1, 2, 3]);
    assert_eq!(arr1.len(), 3);
    assert_eq!(arr1.capacity(), 3);
    assert_eq!(arr1[0], 1);
    assert_eq!(arr1[1], 2);
    assert_eq!(arr1[2], 3);
}

#[test]
fn test_insert() {
    let mut arr = array!(1, 2, 3; 10);
    arr.insert(3, 4);
    assert_eq!(arr, array!(1, 2, 3, 4));
    arr.insert(2, 0);
    assert_eq!(arr, array!(1, 2, 0, 3, 4));
}

#[test]
fn test_len() {
    let arr = array!(1, 2, 3);
    assert_eq!(arr.len(), 3);
}

#[test]
fn test_set_len() {
    let mut arr = array!(1, 2, 3);
    unsafe { arr.set_len(2) };
    assert_eq!(arr.len(), 2);
}

#[test]
fn test_buf() {
    let arr = array!(1, 2, 3);
    let buf = arr.buf();
    assert_eq!(buf, &[1, 2, 3]);
}

#[test]
fn test_buf_mut() {
    let mut arr = array!(1, 2, 3);
    let buf_mut = arr.buf_mut();
    buf_mut[0] = 4;
    assert_eq!(arr.get(0), Some(&4));
}

#[test]
fn test_capacity() {
    let arr: Array<i32, 5> = Array::default();
    assert_eq!(arr.capacity(), 5);
}

#[test]
fn test_is_empty() {
    let arr: Array<i32, 5> = Array::default();
    assert!(arr.is_empty());
}

#[test]
fn test_get() {
    let arr = array!(1, 2, 3);
    assert_eq!(arr.get(1), Some(&2));
}

#[test]
fn test_get_mut() {
    let mut arr = array!(1, 2, 3);
    if let Some(val) = arr.get_mut(1) {
        *val = 4;
    }
    assert_eq!(arr.get(1), Some(&4));
}

#[test]
fn test_last() {
    let arr = array!(1, 2, 3);
    assert_eq!(arr.last(), Some(&3));
}

#[test]
fn test_last_mut() {
    let mut arr = array!(1, 2, 3);
    if let Some(val) = arr.last_mut() {
        *val = 4;
    }
    assert_eq!(arr.last(), Some(&4));
}

#[test]
fn test_first() {
    let arr = array!(1, 2, 3);
    assert_eq!(arr.first(), Some(&1));
}

#[test]
fn test_first_mut() {
    let mut arr = array!(1, 2, 3);
    if let Some(val) = arr.first_mut() {
        *val = 4;
    }
    assert_eq!(arr.first(), Some(&4));
}

#[test]
fn test_push() {
    let mut arr = Array::<i32, 5>::default();
    arr.push(1);
    assert_eq!(arr.len(), 1);
    assert_eq!(arr.get(0), Some(&1));
}

#[test]
fn test_pop() {
    let mut arr = array!(1, 2, 3);
    assert_eq!(arr.pop(), Some(3));
    assert_eq!(arr.len(), 2);
}

#[test]
fn test_iter() {
    let arr = array!(1, 2, 3);
    let mut iter = arr.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_mut() {
    let mut arr = array!(1, 2, 3);
    for val in arr.iter_mut() {
        *val *= 2;
    }
    assert_eq!(arr, array!(2, 4, 6));
}

#[test]
fn test_append() {
    let mut arr1 = array!(1, 2, 3; 5);
    let arr2 = array!(4, 5);
    arr1.append(&arr2);
    assert_eq!(arr1, array!(1, 2, 3, 4, 5));
}

#[test]
fn test_append_slice() {
    let mut arr1 = array!(1, 2, 3; 5);
    let arr2 = [4, 5];
    arr1.append_slice(&arr2);
    assert_eq!(arr1, array!(1, 2, 3, 4, 5));
}

#[test]
fn test_filter() {
    let arr = array!(1, 2, 3, 4, 5);
    let filtered = arr.filter(|&x| x % 2 == 0);
    assert_eq!(filtered, array!(2, 4));
}

#[test]
fn test_map() {
    let arr = array!(1, 2, 3);
    let mapped = arr.map(|&x| x * 2);
    assert_eq!(mapped, array!(2, 4, 6));
}

#[test]
fn test_truncate() {
    let mut arr = array!(1, 2, 3, 4, 5);
    arr.truncate(3);
    assert_eq!(arr, array!(1, 2, 3));
}

#[test]
fn test_as_slice() {
    let arr = array!(1, 2, 3);
    assert_eq!(arr.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_as_mut_slice() {
    let mut arr = array!(1, 2, 3);
    let slice = arr.as_mut_slice();
    slice[0] = 4;
    assert_eq!(arr, array!(4, 2, 3));
}

#[test]
fn test_as_vec() {
    let arr = array!(1, 2, 3);
    assert_eq!(arr.as_vec(), vec![1, 2, 3]);
}

#[test]
fn test_remove() {
    let mut arr = array!(1, 2, 3);
    assert_eq!(arr.remove(1), 2);
    assert_eq!(arr, array!(1, 3));
}

#[test]
fn test_clear() {
    let mut arr = array!(1, 2, 3);
    arr.clear();
    assert_eq!(arr.len(), 0);
}

#[test]
fn test_as_mut_ptr() {
    let mut arr = array!(1, 2, 3);
    let ptr = arr.as_mut_ptr();
    unsafe {
        *ptr = 4;
    }
    assert_eq!(arr.get(0), Some(&4));
}

#[test]
fn test_as_ptr() {
    let arr = array!(1, 2, 3);
    let ptr = arr.as_ptr();
    unsafe {
        assert_eq!(*ptr, 1);
    }
}

#[test]
fn test_as_mut_ptr_range() {
    let mut arr = array!(1, 2, 3);
    let range = arr.as_mut_ptr_range();
    unsafe {
        *range.start = 4;
    }
    assert_eq!(arr.get(0), Some(&4));
}

#[test]
fn test_as_ptr_range() {
    let arr = array!(1, 2, 3);
    let range = arr.as_ptr_range();
    unsafe {
        assert_eq!(*range.start, 1);
    }
}

#[test]
fn test_into_iter() {
    let arr = array!(1, 2, 3);
    unsafe {
        let mut iter = arr.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn test_as_ref() {
    let arr = array!(1, 2, 3);
    let slice: &[i32] = arr.as_ref();
    assert_eq!(slice, &[1, 2, 3]);
}

#[test]
fn test_as_mut() {
    let mut arr = array!(1, 2, 3);
    let slice: &mut [i32] = arr.as_mut();
    slice[0] = 4;
    assert_eq!(arr, array!(4, 2, 3));
}

#[test]
fn test_deref() {
    let arr = array!(1, 2, 3);
    let slice: &[i32] = &arr;
    assert_eq!(slice, &[1, 2, 3]);
}

#[test]
fn test_deref_mut() {
    let mut arr = array!(1, 2, 3);
    let slice: &mut [i32] = &mut arr;
    slice[0] = 4;
    assert_eq!(arr, array!(4, 2, 3));
}

#[test]
fn test_macro() {
    let arr: Array<u8, 10> = array!();
    assert_eq!(arr.len(), 0);
    assert_eq!(arr.capacity(), 10);
    let arr = array!(u8; 10);
    assert_eq!(arr.len(), 0);
    assert_eq!(arr.capacity(), 10);
    let arr = array!(1; 10);
    assert_eq!(arr.len(), 10);
    assert_eq!(arr.capacity(), 10);
    let arr = array!(0,; 10);
    assert_eq!(arr.len(), 1);
    assert_eq!(arr.capacity(), 10);
    let arr: Array<u8, 10> = array!(; 10);
    assert_eq!(arr.len(), 0);
    assert_eq!(arr.capacity(), 10);
    let arr = array!(1, 2, 3);
    assert_eq!(arr.len(), 3);
    assert_eq!(arr.capacity(), 3);
    let arr = array!(1, 2, 3; 10);
    assert_eq!(arr.len(), 3);
    assert_eq!(arr.capacity(), 10);
}
