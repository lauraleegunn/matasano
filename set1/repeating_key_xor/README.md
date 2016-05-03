# implement repeating-key XOR

Here is the opening stanza of an important work of the English language:

>   Burning 'em, if you ain't quick and nimble
>   I go crazy when I hear a cymbal

Encrypt it, under the key "ICE", using repeating-key XOR.

In repeating-key XOR, you'll sequentially apply each byte of the key; 
the first byte of plaintext will be XOR'd against I, the next C, the 
next E, then I again for the 4th byte, and so on.

It should come out to:

```
0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f
```

Encrypt a bunch of stuff using your repeating-key XOR function. Encrypt 
your mail. Encrypt your password file. Your .sig file. Get a feel for 
it. I promise, we aren't wasting your time with this.

# solution

so, first I [implemented repeating-key XOR](src/lib.rs) which was fairly
trivial using rust's `cycle()` method on iterators (that way, the
iterator that iterates over the key cycles around instead of stopping
as it hits the end).

I then wrote a small [command-line tool](src/bin/apply.rs) which can
encrypt cleartext with a given passcode and yield a hex-encoded
text, or it can decrypt a hex-encoded ciphertext with a given passcode
and yield the cleartext. 

using that tool, you can both encrypt the poem:

```bash
./target/debug/apply -e $'Burning \'em, if you ain\'t quick and nimble\nI go crazy when I hear a cymbal' "ICE"

```

and you can decrypt the poem and see that it does in fact work:

```bash
./target/debug/apply -d 0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f ICE
```

# break repeating-key XOR

>   It is officially on, now.
>   
>   *This challenge isn't conceptually hard, but it involves 
>   actual error-prone coding. The other challenges in this 
>   set are there to bring you up to speed. This one is there 
>   to qualify you. If you can do this one, you're probably 
>   just fine up to Set 6.*

There's a [file here](6.txt). It's been base64'd after being encrypted 
with repeating-key XOR.

Decrypt it.

Here's how:

1.  Let `KEYSIZE` be the guessed length of the key; try values from 
    2 to (say) 40.

2.  Write a function to compute the edit distance/Hamming distance 
    between two strings. The Hamming distance is just the number of 
    differing bits. The distance between:

    `this is a test`

    and

    `wokka wokka!!!`

    is 37. Make sure your code agrees before you proceed.

3.  For each KEYSIZE, take the first KEYSIZE worth of bytes, and 
    the second KEYSIZE worth of bytes, and find the edit distance 
    between them. Normalize this result by dividing by KEYSIZE.

4.  The KEYSIZE with the smallest normalized edit distance is probably 
    the key. You could proceed perhaps with the smallest 2-3 KEYSIZE
    values. Or take 4 KEYSIZE blocks instead of 2 and average the distances.

5.  Now that you probably know the KEYSIZE: break the ciphertext 
    into blocks of KEYSIZE length.

6.  Now transpose the blocks: make a block that is the first byte of 
    every block, and a block that is the second byte of every block, and so on.

7.  Solve each block as if it was single-character XOR. You already 
    have code to do this.

8.  For each block, the single-byte XOR key that produces the best 
    looking histogram is the repeating-key XOR key byte for that 
    block. Put them together and you have the key.

This code is going to turn out to be surprisingly useful later on. 
Breaking repeating-key XOR ("Vigenere") statistically is obviously 
an academic exercise, a "Crypto 101" thing. But more people "know how" 
to break it than can actually break it, and a similar technique breaks 
something much more important.

## solution

well, there isn't really all that much to say, the task already
outlines the shape of the code. I started with writing
[hamming distance](src/hamming.rs) calculation code for both
single bytes and arrays of bytes (which ideally have an equal
length, because otherwise it uses the length of the smallest
array).

then I proceeded to write a [`keysize_candidates(data, min, max)`
function](src/lib.rs) which returns an array of keysize candidates 
that are between min and (max-1), sorted so that the most likely 
candidate is up top.

using that code, I then went on to write the actual "cipher breaking"
code, which just takes the first three candidates, and uses
the algorithm to try to decrypt the message using the candidate
as the key length and the code from [single byte XOR](../single_byte_xor)
to do the actual breaking. There is the option to rank the decryptions
produced by the different keysize candidates, but I didn't implement
that because the decryption using the most likely candidate always
returned something useful for me.

i think the most challenging part of this code was actually writing
the transpose function, which I am using for step 8: putting the
transposed block back together. I really struggled with that,
partially because I probably did it way more complicated than I
had to and because `rustc` was so angry at me I had to run to 
the #rust IRC channel and ask for help.
