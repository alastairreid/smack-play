SMACK = smack

# default loop limit
LIMIT := 1

# default unroll limit
UNROLL := 2

check::
	$(SMACK) src/fib.rs --loop-limit $(LIMIT) --unroll $(UNROLL)
	$(SMACK) src/fib.c --loop-limit $(LIMIT) --unroll $(UNROLL)

# End of Makefile
