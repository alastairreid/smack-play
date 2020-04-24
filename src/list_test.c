/****************************************************************
 * Singly linked list
 *
 * Copyright Alastair Reid 2020
 * SPDX-Licence-Identifier: BSD-3-Clause
 ****************************************************************/

#include "stdlib.h"
#include "list.h"
#include "smack.h"

int main()
{
	unsigned n = __VERIFIER_nondet_unsigned_int();
	struct node *l = 0;
	for(unsigned i = 0; i<n; ++i) {
		int x = __VERIFIER_nondet_int();
		l = cons(x, l);
	}

	for(unsigned i = 0; i<n; ++i) {
		assert(l != 0);
		int x = pop(&l);
		assert(x >= 0); // expected to fail
	}

	assert(l == 0);
	return 0;
}

/****************************************************************
 * End
 ****************************************************************/
