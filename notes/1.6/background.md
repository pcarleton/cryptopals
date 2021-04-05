

https://math.stackexchange.com/questions/28955/how-to-break-xor-cipher-with-repeating-key

IC or Chi test

IC = Index of Coincidence

https://en.wikipedia.org/wiki/Index_of_coincidence


The method that crypopals employs seems to be related to the index of coincidence from the link above, but I don't see how... 

This person does a Chi test to determine whether it's english text.

In any event, there doesn't seem to be a good answer for what I'm trying to determine


https://crypto.stackexchange.com/questions/8845/finding-a-keylength-in-a-repeating-key-xor-cipher

> Discover the length of the key by counting coincidences. (See Gaines [GAI44], Sinkov [SIN66].) Trying each displacement of the ciphertext against itself, count those bytes which are equal. If the two ciphertext portions have used the same key, something over 6% of the bytes will be equal. If they have used different keys, then less than 0.4% will be equal (assuming random 8-bit bytes of key covering normal ASCII text). The smallest displacement which indicates an equal key is the length of the repeated key.



http://www.faqs.org/faqs/cryptography-faq/part10/

[GAI44] H. Gaines, Cryptanalysis, a study of ciphers and their
          solution. Dover Publications, 1944.

https://www.amazon.com/Cryptanalysis-Study-Ciphers-Their-Solution/dp/0486200973
https://archive.org/stream/1956Cryptanalysis-AStudyOfCiphersAndTheirSolution/1956+-+Cryptanalysis+%E2%80%93+a+study+of+ciphers+and+their+solution_djvu.txt

https://archive.org/details/1956Cryptanalysis-AStudyOfCiphersAndTheirSolution/page/n1/mode/2up

[SIN66] A. Sinkov, Elementary Cryptanalysis. Math. Assoc. Am. 1966.

https://archive.org/details/elementarycrypta0000sink/page/n7/mode/2up



I'm satisfied that a graphical representation of hamming distances for this application has not really been done.