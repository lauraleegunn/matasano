# matasano crypto challenge [![Build Status](https://travis-ci.org/xfbs/matasano.svg?branch=master)](https://travis-ci.org/xfbs/matasano)

this is my attempt at solving the challenge using rust, to both
learn more about rust and crypto.

see the [website](http://cryptopals.com/) for more information.

## solutions

 -  [set 1](set1/)

## building and testing

there are makefiles which will try to
build all of the solutions, and they
can test then as well.

```bash
make build
make test
```

## todo

 -  in [set1](set1/)/[aes_128_ecb](set1/aes_128_ecb/), i'd like to
    try to actually implement the whole of aes myself, both in
    rust and maybe in C, just to get a feel for it and see how
    it actually works, instead of using openSSL to do the heavy
    lifting.
