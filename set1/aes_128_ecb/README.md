# AES in ECB mode

The Base64-encoded content in [this file](7.txt)
 has been encrypted via AES-128 in ECB mode under the key

```
"YELLOW SUBMARINE".
```

(case-sensitive, without the quotes; exactly 16 characters; 
I like "YELLOW SUBMARINE" because it's exactly 16 bytes long, 
and now you do too).

Decrypt it. You know the key, after all.

Easiest way: use OpenSSL::Cipher and give it AES-128-ECB as the cipher.

## solution

well, the task pretty much gives it away. go to [crates.io](https://crates.io),
dig out the openssl crate (helpfully named `openssl`), add that
to the [`Cargo.toml`](Cargo.toml) file as a dependency, and
off we go. At this moment, I should also mention that it was a
pure joy to use the whole cargo build system because I could
have local dependencies between the different solutions, so
for example `repeating_key_xor` could depend both on 
`single_byte_xor` for the single byte decryption and on `hex_to_base64`
for the hex/base64 encoding and decoding. 

anyways, it unforunately did not work out of the box. I checked
the [github repo](https://github.com/sfackler/rust-openssl) and
it's apparently a known issue. I did the following to properly
install openssl in the first place (the linking step is optional,
btw):

```bash
brew install openssl
brew link --force openssl
```

and then I had to set either 

```
INCLUDE=/usr/local/include
```

to have cargo find it (only works if you `brew link --force`'d it) or

```bash
DEP_OPENSSL_INCLUDE = `brew --prefix openssl`/include
```

which works regardless of wether you `brew link --force`'d it or not.
I put the latter into the `Makefile`, so if on your system
`cargo build` doesn't work and you are sure that you have
openssl installed properly, try `make build`, and if that doesn't
work, find out where you have your headers and set the variables
by hand.

anyways, so [the code](src/lib/openssl-aes-128-ecb.rs) is pretty
much trivial, I parse the path to the base64 encoded file and
the passcode, base64decode the contents of the file, 
decrypt, and dump whatever I got. try it for yourself:

```bash
./target/debug/openssl-aes-128-ecb ./7.txt "YELLOW SUBMARINE"
```

note that the secret has to be exactly 16 bytes long, otherwise
you will get some `panic`s. Also, if after decryption it's not
valid UTF-8 (maybe because you put the secret in wrong), it
dumps it raw (as an array of bytes), so check that out!

# detect AES in ECB mode

In [this file](8.txt) are a bunch of hex-encoded ciphertexts.

One of them has been encrypted with ECB.

Detect it.

Remember that the problem with ECB is that it is stateless 
and deterministic; the same 16 byte plaintext block will 
always produce the same 16 byte ciphertext.

## solution

well, I am interpreting the task so that I am supposed to
check for repetitions of the 16-byte blocks, because if you
have two identical 16-byte blocks and you encrypt them, you
will get the same 16-byte ciphertexts out (unlike in other
encryption modes).

I wrote a [small command-line tool](src/bin/detect-128-ecb.rs)
to basically check for duplicate 16-byte blocks in a file
and say where it found them (and how many duplicates there
are). You can try it like this:

```bash
./target/debug/detect-128-ecb ./8.txt
```
