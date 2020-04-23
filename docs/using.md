# Using SMACK

This is just a quick sketch of what it is like using
the tool.
Detailed examples are in <docs/examples.md>

## Verifying C

There is a simple example in [SMACK's notes on
running](https://github.com/smackers/smack/blob/master/docs/running.md)
that uses a simple [bank account
example](https://github.com/smackers/smack/blob/master/examples/simple/simple.c).
(See also, [more usage
 notes](https://github.com/smackers/smack/blob/master/docs/usage-notes.md).)

In the [simple
example](https://github.com/smackers/smack/blob/master/examples/simple/simple.c),
the `main` function verifies that, for any integers `x`, `y` and `z`
the function `test_account` does not fail any assertions.

This is done by using `__VERIFIER_nondet_int()` to create an arbitrary integer
value.

```C
int main(void) {
  int x = __VERIFIER_nondet_int();
  int y = __VERIFIER_nondet_int();
  int z = __VERIFIER_nondet_int();

  // Check account with nondeterministic values
  test_account(x, y, z);
  return 0;
}
```

SMACK also has some support for contracts using the
[smack-contracts.h](https://github.com/smackers/smack/blob/master/share/smack/include/smack-contracts.h)
header file
which provides `requires`, `ensures`, `invariant`, `forall` and `exists`.
(A 'todo' notes that `old` and `result` are not yet supported.)
[Some of the
tests](https://github.com/smackers/smack/tree/master/test/c/contracts)
demonstrate how to use this.
