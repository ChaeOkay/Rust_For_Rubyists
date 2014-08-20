#Notes

####Crates
-  Running `rustc` compiles a single crate.
-  A "Root crate" is the highest level module in a nested module hierarchy.
-  Qualifying a module requires declaring a path.
-  Any path starting with `::` is a global path relative to the root crate.
-  `fn`, `struct`, `static`, `mod`, and others by default a private. Public functions start with `pub`.
-  Visibility restrictions exist at the module boundaries!

####Libraries
-  The standard library is automatically loaded during compile time.
-  Running `rustc` inserts the decleration `extern crate std;` in the root crate.
-  Running `rustc` inserts `use std::prelude::*;` into every module body.
