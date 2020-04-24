// This file can be compiled in two ways:
// - in verification mode using symbolic values
// - in test mode using a for loop to generate concrete values
#ifdef VERIFY
#include "smack.h"
#else
#include "stdio.h"
#endif

unsigned long fib(unsigned long x) {
    unsigned long a = 0, b = 1;
    for (unsigned long i=0; i<=x; ++i) {
        unsigned long tmp = a;
        a = b;
        b = a + tmp;
    }
    return a;
}

int main() {
#ifdef VERIFY
    unsigned long n = __VERIFIER_nondet_unsigned_long();
    assert(fib(n) >= n);
#ifdef TEST_FAILURE
    // We know the following is not valid - it lets us test what failures look like
    // assert(fib(n) < 2);
#endif
#else
	for(unsigned long i = 0; i < 10; ++i) {
		printf("fib(%lu) = %lu\n", i, fib(i));
	}
#endif
    return 0;
}
