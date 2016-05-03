TARGETS = \
	hex_to_base64 \
	fixed_xor \
	single_byte_xor \
	repeating_key_xor \
	aes_128_ecb

test: $(TARGETS)

build: $(TARGETS)

clean: $(TARGETS)

.PHONY: clean test build $(TARGETS)

$(TARGETS):
	cd $@ && make $(MAKECMDGOALS)
