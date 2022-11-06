# Ownership and Borrowing in Rust - Notes

- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped
- If you have a mutable reference to a value, you can have no other references
  to that value in the same scope.
- References must always be valid
- At any given time, you can have either one mutable reference or any number of
  immutable references
