# single-byte XOR cipher

The hex encoded string:

```
1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
```

... has been XOR'd against a single character. Find the key, decrypt the message.

You can do this by hand. But don't: write code to do it for you.

How? Devise some method for "scoring" a piece of English plaintext. 
Character frequency is a good metric. Evaluate each output and 
choose the one with the best score.

## solution

to do this, I first wrote [a small ruby script](parse_letter_frequency_data.rb)
to parse a dump of [ASCII letter frequency data](letter_frequency_data.txt)
that I shamelessly took from [here](http://fitaly.com/board/domper3/posts/136.html).

then I took the parsed letter frequency data and used that to write a
[function that can parse text and determine the likelyhood of that
text being enlish](src/scoring.rs), using the frequency data.
surprisingly, `e` is not the most common letter, but ` ` (space) is.

all [my decryption method](src/lib.rs) does is iterate thru the bytes `0x00...0xFF`,
uses them to decrypt the message, and ranks the decryptions according to
their likelyhood of being english. the decrypted text with the highest likelyhood
is then usually the plaintext.

I then built a small [commandline tool](lib/bin.rs) to decrypt either
a hex encoded string or all hex encoded strings from a file. after having
built everything, run it to see the decrypted hex string:

```bash
make build
./target/debug/single_byte_xor_decrypter --hex 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
```

# detect single-character XOR

One of the 60-character strings in [this](challenge4.txt) file has been 
encrypted by single-character XOR.

Find it.

(Your code from #3 should help.)

## solution

using the same tool as for decrypting the XOR cipher, it is possible
to decrypt the file:

```bash
make build
./target/debug/single_byte_xor_decrypter --file challenge4.txt
```

it will spit out a lot of random data, but if you look hard enought you can 
find the one plain english sentence. I could have built it so that it
re-ranks all the decrypted data against to find which one is most likely
english and not random noise, but since it's easy to just manually scan
through, I chose to save myself the trouble.
