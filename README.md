`pb-rs -d pb *.proto` to generate `pb/*.rs` files.

`cargo run`:

```
cargo run
   Compiling foo v0.1.0 (/home/marko/.emacs.d/rust-playground/at-2020-06-20-233607)
error[E0412]: cannot find type `Common` in module `b`
  --> src/pb/a.rs:18:27
   |
18 |     pub common: Option<b::Common>,
   |                           ^^^^^^ not found in `b`
   |
help: possible candidate is found in another module, you can import it into scope
   |
12 | use crate::pb::common::Common;
   |

error[E0412]: cannot find type `Common` in module `b`
  --> src/pb/a.rs:27:65
   |
27 |                 Ok(10) => msg.common = Some(r.read_message::<b::Common>(bytes)?),
   |                                                                 ^^^^^^ not found in `b`
   |
help: possible candidate is found in another module, you can import it into scope
   |
12 | use crate::pb::common::Common;
   |

error[E0283]: type annotations needed
  --> src/pb/a.rs:18:5
   |
18 |     pub common: Option<b::Common>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
   |
   = note: cannot resolve `_: std::default::Default`
   = note: required by `std::default::Default::default`

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0283, E0412.
For more information about an error, try `rustc --explain E0283`.
error: could not compile `foo`.

To learn more, run the command again with --verbose.
```
