# Issues

*(Note that the following issues may well be my fault – I might have
built/installed/configured/used SMACK in the wrong way.)*

At the moment, I am having some problems

- SMACK reports `llvm2bpl: warning: unsoundly ignoring unmodeled operation landingpad clauses;`

  This is unfortunate because we want sound results!

- SMACK generates Corral code containing a type error if you use `--bit-precise`
  in [src/fib.rs](/src/fib.rs).

- SMACK replaces the standard library with
  [share/smack/lib/smack.rs](https://github.com/smackers/smack/blob/master/share/smack/lib/smack.rs)
  which implements a subset of the standard library.
  A quick glance suggests that it supports Vec, Box and some iterators.
  So more work is likely needed to support more of the Rust
  standard library.


- rustc and smack typecheck the code differently in
  in [src/fib.rs](/src/fib.rs).

  With the code currently commited, rustc reports

  ```
  15 |     let mut cache = vec![0; (n+1) as u64];
     |                             ^^^^^^^^^^^^ expected usize, found u64
  ```

  If I change the code to fix this type error, rustc is happy but smack reports

  ```
  15 |     let mut cache = vec![0; (n+1) as usize];
     |                             ^^^^^^^^^^^^^^ expected u64, found usize
  ```

  (I suspect that this is because SMACK replaces the standard
  library?)

- Different `--unroll` values are needed for C and Rust.

  This is not an error but it is worth being aware that, although
  [src/fib.rs](/src/fib.rs) and
  [src/fib.c](/src/fib.c) are fairly similar and contain the same
  deliberately false assertion,
  SMACK will find the error in the C code with `--unroll 2`
  but requires `--unroll 5` to find the error in the Rust code.

  Alas, `--unroll 5` is significantly slower than `--unroll 2`.

- It is not obvious how to use SMACK with [Cargo].
  This is not a problem for small tests but will clearly be an
  issue if you try to apply SMACK to a large codebase.

- SMACK does not appear to support modular verification for
  Rust yet.
  In particular, the file [share/smack/lib/smack.rs]
  does not define `requires`, `ensures` and `invariant` macros
  or similar.
  It is not obvious how hard it would be to fix this.

I suspect that several of these issues will affect other Rust
verification tools.
In particular,

1. Most tools will require an annotated, simplified or
   replacement standard library.
   The necessary changes are
   - Replace highly optimized implementations with simpler
     versions that make verification easier.
     (A bit like a manual version of what was done
     in the [-Overify paper].)

     It is not clear whether it will be possible for several
     different tools to share a single modified library

   - Add some SMACK specific implementations
     of functions/macros like `assert`, `assert_eq`, `assume`
     and `nondet` (to create a symbolic value).

     Some of this is probably SMACK-specific.

2. All tools will require some integration with [Cargo].
   This would probably take the form of a [Cargo extension]
   that behaves like [Cargo test].

   Hopefully this can be done in a way that supports
   several verification tools.


[Cargo]: https://doc.rust-lang.org/cargo/
[Cargo extension]: https://doc.rust-lang.org/book/ch14-05-extending-cargo.html
[Cargo test]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
[share/smack/lib/smack.rs]: https://github.com/smackers/smack/blob/master/share/smack/lib/smack.rs
[share/smack/include/smack-contracts.h]: https://github.com/smackers/smack/blob/master/share/smack/include/smack-contracts.h
[-Overify paper]: https://dslab.epfl.ch/pubs/overify.pdf
