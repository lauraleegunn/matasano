CARGO = cargo

# this is needed to get the openssl crate to
# compile on os x after having installed it
# with homebrew.
DEP_OPENSSL_INCLUDE = `brew --prefix openssl`/include

test:
	$(CARGO) test

build:
	$(CARGO) build

clean:
	$(CARGO) clean
