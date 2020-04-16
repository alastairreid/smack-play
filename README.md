# SMACK-play

Experiments in using the [Smack verification tool](http://smackers.github.io/)
with C and Rust.

These are mostly just explorations of the tool but I am making them available
in case they are useful to others.

## About SMACK


> SMACK is both a modular software verification toolchain and a self-contained
> software verifier. It can be used to verify the assertions in its input
> programs. In its default mode, assertions are verified up to a given bound on
> loop iterations and recursion depth; it contains experimental support for
> unbounded verification as well. Under the hood, SMACK is a translator from the
> LLVM compiler's popular intermediate representation (IR) into the Boogie
> intermediate verification language (IVL). Sourcing LLVM IR exploits an
> increasing number of compiler front-ends, optimizations, and analyses.
> Targeting Boogie exploits a canonical platform which simplifies the
> implementation of algorithms for verification, model checking, and abstract
> interpretation. – [SMACK homepage](http://smackers.github.io/)

So, it supports many different languages,
it is fairly low effort to start using,
but, because it is a _bounded verifier_, it proves that
a program does not have bugs _up to some bound/depth_ but it makes
no guarantees beyond that bound.

## Table of Contents

1. [Installing SMACK](docs/installation.md)
2. [Using SMACK](docs/using.md)
