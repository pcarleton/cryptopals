## Prompt

There's a file here. It's been base64'd after being encrypted with repeating-key XOR.

Decrypt it.

Here's how:

1. Let KEYSIZE be the guessed length of the key; try values from 2 to (say) 40.
2. Write a function to compute the edit distance/Hamming distance between two strings. The Hamming distance is just the number of differing bits. The distance between:

> this is a test

and

> wokka wokka!!!

is 37. Make sure your code agrees before you proceed.

3. For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, and find the edit distance between them. Normalize this result by dividing by KEYSIZE.
4. The KEYSIZE with the smallest normalized edit distance is probably the key. You could proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances.

Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.

Now transpose the blocks: make a block that is the first byte of every block, and a block that is the second byte of every block, and so on.

Solve each block as if it was single-character XOR. You already have code to do this.

For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block. Put them together and you have the key.

This code is going to turn out to be surprisingly useful later on. Breaking repeating-key XOR ("Vigenere") statistically is obviously an academic exercise, a "Crypto 101" thing. But more people "know how" to break it than can actually break it, and a similar technique breaks something much more important.


## Notes

Hamming Distance

https://en.wikipedia.org/wiki/Hamming_distance

XOR gives you the bits that are different

Hamming weight gives you the number of 1 bits in a value.

Wegner way:
while val > 0: sum++; val = val & (val - 1);

It works becaase if there's a 1 in the last place, it eats that 1.

IF there's not a 1... then:

110 & 101 = 100 so it still eats it.


110000 & 10111 = 100000, so still works, regardless of what's happening, it will eat up until the least significant 1.