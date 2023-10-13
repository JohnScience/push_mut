# push_mut

[![Crates.io](https://img.shields.io/crates/v/push_mut)](https://crates.io/crates/push_mut)
[![Downloads](https://img.shields.io/crates/d/push_mut.svg)](https://crates.io/crates/push_mut)
[![Documentation](https://docs.rs/push_mut/badge.svg)](https://docs.rs/push_mut)
[![License](https://img.shields.io/crates/l/push_mut)](https://crates.io/crates/push_mut)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/push_mut/status.svg)](https://deps.rs/repo/github/JohnScience/push_mut)

Push a value to the back of the vector, and return a mutable reference to it.

## Example

```rust
use push_mut::PushMut;

fn main() {
    let mut v = Vec::with_capacity(1);
    let last = v.push_mut(1);
    assert_eq!(*last, 1);
    *last = 2;
    assert_eq!(*last, 2);   
}
```
