SMACK = smack

# default loop limit
LIMIT := 1

# default unroll limit
UNROLL := 2

check::
	$(SMACK) --modular src/fact.c
	$(SMACK) src/list.c src/list_test.c --entry-point test_list --unroll $(UNROLL)
	$(SMACK) src/fib.rs --loop-limit $(LIMIT) --unroll $(UNROLL)
	$(SMACK) src/fib.c --loop-limit $(LIMIT) --unroll $(UNROLL)

clean::
	$(RM) src/smack.rs
	$(RM) error.bpl

# End of Makefile
