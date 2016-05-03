# fixed XOR

Write a function that takes two equal-length buffers and 
produces their XOR combination.

If your function works properly, then when you feed it the string:

```
1c0111001f010100061a024b53535009181c
```

... after hex decoding, and when XOR'd against:

```
686974207468652062756c6c277320657965
```

... should produce:

```
746865206b696420646f6e277420706c6179
```

## solution

I [implemented this](src/lib.rs) in about 4 lines of code in
rust, it's very simple to do. 

the only problem with the solution is that it doesn't enforce
that the buffers are of equal length, so when you pass it two
buffers of different length, it uses the length of the smaller
of the two buffers (one of the perks of rust's `.zip()`
function on interators). 
