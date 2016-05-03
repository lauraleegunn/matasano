# convert hex to base64

The string:

```
49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
```

should produce:

```
SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
```

So go ahead and make that happen. You'll need this code for later
exercises.

>   Cryptopals rule
>   
>   *Always operate on raw bytes, never on encoded strings. Only 
>   use hex and base64 for pretty-printing.*

## solution

I have implemented [hex encoding and decoding functions](src/hex.rs)
as well as [base64 encoing and decoding functions](src/base64.rs) in
rust, and there is a [test](src/lib.rs) to make sure this works
alright.

The base64 code is ugly and seriously lacks error handling, but 
it works and it's only meant to be a proof-of-concept anyways.
