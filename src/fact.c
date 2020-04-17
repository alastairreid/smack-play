#include <smack-contracts.h>
#include <smack.h>

unsigned long fact(unsigned long x) {
	requires(x >= 0);
	unsigned long r = 1;
	ensures(r > 0);
	for(int i = 1; i <= x; ++i) {
		invariant(r > 0 && i > 0);
		r *= i;
	}
	return r;
}
