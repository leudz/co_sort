# co_sort
Sort multiple arrays given a permutation.

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/v/co_sort.svg)](https://crates.io/crates/co_sort)
[![Documentation](https://docs.rs/co_sort/badge.svg)](https://docs.rs/co_sort)

The simplest way to use this crate is the co_sort! macro, it will sort the first array and swap elements of the others in order to mimic the changes in the first array. \
Usefull when you have multiple slices with an implicit relation.
```rust
#[macro_use] extern crate co_sort;
use co_sort::*;
let mut names = ["Diego", "Maia", "Luciana", "Bruno", "Astrid", "Thierry"];
let mut ages =  [  73,      88,      21,        47,      4,        62    ];
// We want to sort the names but keep the ages synced
co_sort![names, ages];
assert_eq!(names, ["Astrid", "Bruno", "Diego", "Luciana", "Maia", "Thierry"]);
assert_eq!(ages,  [   4,       47,      73,       21,       88,      62    ]);
```
If you want more control you can use the functions co_sort and co_sort_stable, the macro uses co_sort internally. \
co_sort_stable allocates O(n) memory and requires the types to implement Clone while co_sort is in place and doesn't require any trait. \
Performance wise co_sort scale well with the number of arrays but not with their size and co_sort_stable is the opposite.
```rust
# use co_sort::*;
let mut names = ["Diego", "Maia", "Luciana", "Bruno", "Astrid", "Thierry"];
let mut ages =  [  73,      88,      21,       47,       4,        62    ];
let permutation = Permutation::from(names.as_ref());
permutation.co_sort((names.as_mut(), ages.as_mut()));
// or permutation.co_sort_stable((names.as_mut(), ages.as_mut()));
assert_eq!(names, ["Astrid", "Bruno", "Diego", "Luciana", "Maia", "Thierry"]);
assert_eq!(ages,  [   4,       47,      73,       21,       88,      62    ]);
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.