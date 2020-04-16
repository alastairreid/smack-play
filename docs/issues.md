# Issues

(Note that the following issues may well be my fault – I might have
built/installed/configured/used SMACK in the wrong way.)

At the moment, I am having some problems

- SMACK reports `llvm2bpl: warning: unsoundly ignoring unmodeled operation landingpad clauses;`

  This is unfortunate because we want sound results!

- SMACK generates Corral code containing a type error if you use `--bit-precise`
  in [src/fib.rs](src/fib.rs).

- rustc and smack typecheck the code differently in
  in [src/fib.rs](src/fib.rs).

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

- Different `--unroll` values are needed for C and Rust.

  This is not an error but it is worth being aware that, although
  [src/fib.rs](src/fib.rs) and
  [src/fib.c](src/fib.c) are fairly similar and contain the same
  deliberately false assertion,
  SMACK will find the error in the C code with `--unroll 2`
  but requires `--unroll 5` to find the error in the Rust code.

  Alas, `--unroll 5` is significantly slower than `--unroll 2`.
