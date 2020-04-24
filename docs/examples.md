# SMACK examples

This is a short summary of the examples that I have tried so far.

- C
  - [Simple C example](#simple-c-example)
  - [Linked lists](#linked-lists)
  - [Modular verification](#modular-verification)
- Rust
  - [Simple Rust example](#simple-rust-example)
  - [Rust test harness](#rust-test-harness)

## C examples

### Simple C example

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

### Linked lists

Code: [src/list.h](/src/list.h) [src/list.c](/src/list.c) [src/list_test.c](/src/list_test.c)

Example of verifying some linked list code.
It contains an assertion that is known to be false (that all list entries are
non-negative).

This produces a trace like this.
(I have suppressed a lot of lines of the form "filename(line): Thread=1" and
lines involving the file `local/share/smack/lib/smack.c`.)

```
SMACK program verifier version 2.4.1
local/share/smack/lib/smack.c(45,1): This assertion can fail
Execution trace:
    src/list_test.c(16,15):	Thread=1  smack:entry:test_list = -12387
    src/list_test.c(16,15):	Thread=1  CALL __VERIFIER_nondet_unsigned_int
    src/list_test.c(16,15):	Thread=1  RETURN from __VERIFIER_nondet_unsigned_int
    src/list_test.c(16,15):	Thread=1  smack:ext:__VERIFIER_nondet_unsigned_int = 1
    src/list_test.c(16,15):	Thread=1  n = 1
    src/list_test.c(19,11):	Thread=1  CALL __VERIFIER_nondet_int
    src/list_test.c(19,11):	Thread=1  RETURN from __VERIFIER_nondet_int
    src/list_test.c(19,11):	Thread=1  smack:ext:__VERIFIER_nondet_int = -297
    src/list_test.c(19,11):	Thread=1  x = -297
    src/list_test.c(20,7):	Thread=1  CALL cons
    src/list.c(15,19):	Thread=1  cons:arg:x = -297
    src/list.c(15,19):	Thread=1  CALL malloc
    src/list.c(15,19):	Thread=1  RETURN from malloc
    src/list_test.c(20,7):	Thread=1  RETURN from cons
    src/list_test.c(18,27):	Thread=1  i = 1
    src/list_test.c(25,11):	Thread=1  CALL pop
    src/list.c(43,20):	Thread=1  data = -297
    src/list.c(45,2):	Thread=1  CALL free_
    src/list.c(45,2):	Thread=1  RETURN from free_
    src/list_test.c(25,11):	Thread=1  RETURN from pop
    src/list_test.c(25,11):	Thread=1  x = -297
    src/list_test.c(26,3):	Thread=1  CALL __VERIFIER_assert
    src/list_test.c(26,3):	Thread=1  RETURN from __VERIFIER_assert

SMACK found an error.
```

This is a trace of every line of code in the program
including the values chosen for the symbolic variables `n` and `x`.

Note: the trace clearly contains all the information that I need to
debug the failure but I find it a bit awkward.
I wonder whether there is a tool that helps me navigate long traces?
An obvious thing to try would be to suppress calls to "cons" and
"pop" and allow those to be expanded interactively.
This tool seems so obvious that it must already exist?

There is a `--replay` option to "enable reply of error trace with test
harness".
It generates a file `replay-exe`.

```
$ smack list.c list_test.c --unroll 4 --replay
  SMACK program verifier version 2.4.1
  local/share/smack/lib/smack.c(45,1): This assertion can fail
  Execution trace:
    [omitted]
  Attempting to replay error trace.
  Generated replay harness: replay-harness.c
  Generated replay executable: replay-exe
  Error-trace replay successful.
  SMACK found an error.
$ ./replay-exe
  error reached!
```

### Modular verification

Code: [src/fact.c](/src/fact.c)

Boogie's C frontend supports addition of requires/ensures and invariants and
the flag `--modular`.
This is a small test of using this functionality.

The key difference from other examples is that you do not need to pick a
suitable bound for unrolling because loops do not need to be unrolled.

It is not clear whether Rust supports the requires/ensures/invariant features
needed for modular verification.


## Rust examples

### Simple Rust example

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


### Rust test harness

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

