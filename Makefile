SMACK = smack

# default loop limit
LIMIT := 1

# default unroll limit
UNROLL := 2

# hacky command to make traces more readable
CLEAN = grep -v 'Thread=1\s*$$' | grep -v smack/lib/smack.c

check::
	$(SMACK) --modular src/fact.c
check::
	$(SMACK) src/fib.rs --loop-limit $(LIMIT) --unroll $(UNROLL)
check::
	$(SMACK) --clang-options=-DVERIFY src/fib.c --loop-limit $(LIMIT) --unroll $(UNROLL)

# check that failing examples can be replayed
replay::
	- smack src/list.c src/list_test.c --unroll 4 --replay | $(CLEAN)
	- ./replay-exe
replay::
	- $(SMACK) src/fib.c --clang-options=-DTEST_FAILURE --loop-limit $(LIMIT) --unroll $(UNROLL) --replay | $(CLEAN)
	- ./replay-exe

clean::
	$(RM) src/smack.rs
	$(RM) error.bpl
	$(RM) src/replay-exe src/replay-harness.c
	$(RM) replay-exe replay-harness.c

# End of Makefile
