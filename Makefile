SETS = set1

all: $(SETS)

clean: $(SETS)

test: $(SETS)

.PHONY: clean test all $(SETS)

$(SETS):
	cd $@ && make $(MAKECMDGOALS)
