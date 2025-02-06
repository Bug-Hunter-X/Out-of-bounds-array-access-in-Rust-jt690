# Out-of-bounds array access in Rust

This repository demonstrates a common error in Rust: accessing an array or vector element using an index that is out of bounds.  This leads to a panic at runtime.

The `bug.rs` file contains the buggy code. The `bugSolution.rs` file shows how to fix this error by checking the index before accessing the element or using safe methods.