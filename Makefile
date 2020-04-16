SMACK = smack

# default loop limit
LIMIT := 1

# default unroll limit
UNROLL := 1

check::
	$(SMACK) src/fib.rs --loop-limit $(LIMIT) --unroll $(UNROLL)

# End of Makefile
