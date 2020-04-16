#include "smack.h"

unsigned long fib(unsigned long x) {
    unsigned long a = 0, b = 1;
    for (unsigned long i=0; i<x-1; i++) {
        unsigned long tmp = a;
        a = b;
        b = a + tmp;
    }
    return b;
}

int main() {
    unsigned long n = __VERIFIER_nondet_unsigned_long();
    assert(fib(n) >= n);
    assert(fib(n) < 2);
    return 0;
}
