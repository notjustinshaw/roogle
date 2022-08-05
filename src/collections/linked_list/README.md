### A mutable singly-linked list

The data type we will add to the list will be a wrapper around an int.

```rust
pub struct {
  pub u64 num;
};
```

The linked list must have the following functionality:

- create a new linked list (should be empty, head is null)
- insert 100 elements into the list (verify each add increases size)
- sort the list (given a comparator)
- remove the first element (verify it was the correct -sorted- value)
- make an iterator over the list (verify not null)
- use the iterator to peek at the next element (verify its value)
- move the iterator (verify the new value)
- loop and iterate past the end
- clean up
