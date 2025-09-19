# Testing

Instructions for how to run the tests are in the readme.

## Coverage

- todo

## What was tested?

### Huffman

- Compressing and decompressing an ascii string, yields identical data. (compression, not the archive file format)
- Compressing and decompressing a binary file, yields identical data. (compression, not the archive file format)

- Tree builder can correctly count bytes in a byte slice.
- Tree builder can correctly count bytes in a byte slice (ascii text).
- Tree builder can correctly join the two least frequent nodes when building hierarchy from a set of predermined sample nodes.
- Tree builder can correctly build an optimal tree from a set of predermined sample nodes.
- A predetermined sample tree yields correct lookup table.

- Huffman code struct string conversion returns expected values, for debug purposes.
- Huffman code struct can not be created with unused bits set.

- Code writer - todo