# Rust Vector Access and Error Handling

This example demonstrates a common error in Rust when accessing elements in a vector: using `unwrap()` without proper error handling.  The `unwrap()` method will cause a panic if the element does not exist, leading to program termination.  The solution showcases safer alternatives using pattern matching or `.get()` for robust code.

## Bug

The `bug.rs` file shows the problematic code that uses `unwrap()` when accessing the first element of the vector. If the vector is empty, this will cause a panic.

## Solution

The `bugSolution.rs` file demonstrates two safer methods to access vector elements, preventing panics:

1. Using a `match` statement to handle the `None` case elegantly.
2. Using the `get()` method and checking if it returns `Some` value.

This illustrates best practices for handling potential errors when working with vectors or other data structures in Rust.