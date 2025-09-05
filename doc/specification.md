# Specification

- Programming language: Rust
- Peer review programming languages: Python, C(++)
- Peer review languages: Finnish, English
- Program: CS / TKT
- Algorithms: LZ77, Huffman, DEFLATE

The project will implement and compare both compression and decompression of binary data, using different algorithms. Chosen algorithms are LZ77 and Huffman coding, as well as DEFLATE, which combines the other two. Measured aspects are speed and compression ratio.

The program will accept a one or more files, and pack them into an archive, compressed using specified compression. It can also unpack said archives into the original files. 

## Estimating Complexities

The time complexity of building a Huffman tree is O(*n* log *n*), where *n* is the number of unique values in the tree. Intuitively the tree doesn't seem like a very significant portion of the time though. After that, the time complexity improves to O(*n*) for a simple lookup of each byte. The space complexity is similarly higher for the tree: O(*n*) where *n* is unique values. The rest could operate one byte at a time for O(1) space complexity.

From what I understand, the time complexity of compressing LZ77 would be an abysmal O(*n*^2) where *n* is the length, if the size of the search window wasn't limited. The fixed size could reduce both complexities to O(*n*).

## The Core

The core of the project is to pack and unpack binary data. Reading and writing files, as well as messing with the archive container format is secondary. 

## Sources
    - https://en.wikipedia.org/wiki/Deflate
    - https://en.wikipedia.org/wiki/LZ77_and_LZ78
    - https://en.wikipedia.org/wiki/Huffman_coding
    - LLM usage: None whatsoever