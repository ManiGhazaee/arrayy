# arrayy

Stack-allocated fixed-size array with useful methods on top of Rust's `[T; L]` type.

## Methods

- [`len`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.len)
- [`set_len`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.set_len)
- [`buf`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.buf)
- [`buf_mut`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.buf_mut)
- [`capacity`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.capacity)
- [`is_empty`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.is_empty)
- [`get`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.get)
- [`get_mut`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.get_mut)
- [`get_unchecked`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.get_unchecked)
- [`get_unchecked_mut`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.get_unchecked_mut)
- [`last`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.last)
- [`last_mut`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.last_mut)
- [`first`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.first)
- [`first_mut`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.first_mut)
- [`from`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.from)
- [`push`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.push)
- [`pop`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.pop)
- [`iter`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.iter)
- [`iter_mut`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.iter_mut)
- [`into_iter`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.into_iter)
- [`append`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.append)
- [`filter`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.filter)
- [`map`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.map)
- [`truncate`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.truncate)
- [`as_slice`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.as_slice)
- [`as_mut_slice`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.as_mut_slice)
- [`as_vec`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.as_vec)
- [`insert`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.insert)
- [`remove`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.remove)
- [`clear`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.clear)
- [`as_mut_ptr`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.as_mut_ptr)
- [`as_ptr`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.as_ptr)
- [`as_mut_ptr_range`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.as_mut_ptr_range)
- [`as_ptr_range`](https://docs.rs/arrayy/latest/arrayy/struct.Array.html#method.as_ptr_range)

## Examples

### Creating an Array

```rust
use arrayy::{array, Array};

let a = array![u8; 10]; // empty array (len = 0) with capacity = 10
// same as
let a: Array<u8, 10> = Array::default();

let b = array![1u8, 2, 3; 10]; // array with 3 elements (len = 3) and capacity = 10
// same as
let b = Array::<u8, 10>::from(&[1, 2, 3]);

let c = array![1u8, 2, 3]; // array with 3 elements (len = 3) and capacity = 3
// same as
let c = Array::<u8, 3>::from(&[1, 2, 3]);
```

### Basic Operations

```rust
let mut arr = array![1, 2, 3; 10];
arr.push(4);
assert_eq!(arr.len(), 4);
assert_eq!(arr.pop(), Some(4));
assert_eq!(arr.len(), 3);
```

### Accessing Elements

```rust
let arr = array![1, 2, 3];
assert_eq!(arr[1], 2);
assert_eq!(arr.get(1), Some(&2));
assert_eq!(arr.first(), Some(&1));
assert_eq!(arr.last(), Some(&3));
```

### Iterating Over Elements

```rust
let arr = array![1, 2, 3];
for val in arr.iter() {
    println!("{}", val);
}

let mut arr = array![1, 2, 3];
for val in arr.iter_mut() {
    *val *= 2;
}
assert_eq!(arr, array![2, 4, 6]);
```

### Filtering and Mapping

```rust
let arr = array![1, 2, 3, 4, 5];
let filtered = arr.filter(|&x| x % 2 == 0);
assert_eq!(filtered, array![2, 4]);

let mapped = arr.map(|&x| x * 2);
assert_eq!(mapped, array![2, 4, 6, 8, 10]);
```

### Appending Arrays

```rust
let mut arr1 = array![1, 2, 3; 5];
let arr2 = array![4, 5];
arr1.append(&arr2);
assert_eq!(arr1, array![1, 2, 3, 4, 5]);
```
