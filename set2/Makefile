TARGETS = pkcs7

test: $(TARGETS)

build: $(TARGETS)

clean: $(TARGETS)

.PHONY: clean test build $(TARGETS)

$(TARGETS):
	cd $@ && make $(MAKECMDGOALS)
