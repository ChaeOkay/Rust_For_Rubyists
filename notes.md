#Notes

####Strings
- `Str` is a heap allocated string with a dynamic length
- `&str` immutable view into string (pointer?), a literal
- `to_str()` turns a `&str` into a `Str` (conversion method?)
- An application for `to_str()` is when a variable can be string or a num, either of which need to be printed as strings.

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
