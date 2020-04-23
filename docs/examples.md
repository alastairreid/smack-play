# SMACK examples

This is a short summary of the examples that I have tried
so far.

- [Simple C example](#simple-c-example)
- [Modular verification](#modular-verification)
- [Simple Rust example](#simple-rust-example)
- [Rust test harness](#rust-test-harness)

## Simple C example

Code: [src/fib.c](/src/fib.c)

Example of verifying a small C function (fibonacci).
This is just a small demo of using the tool.
It contains two assertions: one (I believe) is always correct and
the other is deliberately incorrect (so that I can see what bug reports look
like).

Probably the most important part is the command (in [Makefile](/Makefile))

```
smack src/fib.c --loop-limit 1 --unroll 2
```

The `--unroll` part causes the loop to be unrolled twice.
This is necessary because the bug will only show up if the loop
in `fib` goes round at least twice.
(I don't think the `--loop-limit` is important here.)


## Modular verification

Code: [src/fact.c](/src/fact.c)

Boogie's C frontend supports addition of requires/ensures and invariants and
the flag `--modular`.
This is a small test of using this functionality.

The key difference from other examples is that you do not need to pick a
suitable bound for unrolling because loops do not need to be unrolled.

It is not clear whether Rust supports the requires/ensures/invariant features
needed for modular verification.

## Simple Rust example

Code: [src/fib.rs](/src/fib.rs)

This example is similar to [src/fib.c](/src/fib.c) except that it is written
in Rust.
(This example comes from the [SMACK/Rust paper](https://soarlab.org/publications/2018_atva_bhr/).)

In the process of trying this example, I found [some issues](/docs/issues.md)
where the code is typechecked differently depending on whether we are
using rustc or using smack.
(It is possible that this is because
<https://github.com/smackers/smack/blob/master/share/smack/lib/smack.rs>
defines `Vec<>` and maybe gives it a non-standard type?)


## Rust test harness

Code: [rust/fib/src](/rust/fib/src)

This example does not work yet.

Small example of separating the test harness from the
code being verified.

One thing that this highlighted for me was that, ideally,
we would integrate verification into `cargo` in the same
way that testing is integrated.

Also, it was not clear to me exactly what had been tested
because, while putting this example together, I ran it
in a way that took a long time but, as far as I can tell,
did not actually test what I was expecting.
What are the equivalent of coverage metrics for testing?

