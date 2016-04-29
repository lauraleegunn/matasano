SETS = set1

test: $(SETS)

clean: $(SETS)

build: $(SETS)

.PHONY: clean test build $(SETS)

$(SETS):
	cd $@ && make $(MAKECMDGOALS)
