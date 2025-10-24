# Implementation

## Structure

For those unfamiliar with Rust, a module in a `mod.rs` file is at the folder's level. `name/mod.rs` is equivalent to `name.rs` in the hierarcy. The main function can be found from the descriptively named `src/main.rs`.

Application logic in `main.rs` suffers from poor planning and lazy coding, but the algorithms and their relevant data structures themseves are arranged into descriptively named modules and are relatively well encapsulated. All structures are housed in submodules named after them (with exception of IO mods, which house both a reader and a writer).

When creating an archive, the program will read the entire file into memory, and then begin writing a new archive. When uncompressing an archive, the program will first unpack it into memory, and then create and write into the target file.

To make the application easier to use, the data is wrapped into a simple container header that tells the program which algorithm to use for unpacking.

## Complexities

### Huffman Coding

Huffman coding involves multiple parts with different complexities.

- **Frequencies:** Iterate over the tree: O(*n*). 
- **Building the tree:** Assembling the huffman tree has a worse time complexity, but this makes little difference -- testing with [cargo flamegraph](https://github.com/flamegraph-rs/flamegraph) shows that the time spent on generating the tree was under 2% for every sample file. This makes sense as the maximum number of unique values in a byte is 256, which is insignificant compared to number of bytes even in the smallest file. 
- **Encoding and decoding:** Time complexity of encoding and decoding data is O(*n*), as the algorithm iterates over the bytes, checking them against a hashmap*.

Space complexity is O(*n*). Memory contains uncompressed data in its entirety, and the Huffman tree*.

*The length of the Huffman tree and therefore the derived hashmap are capped, by the number of unique values in a byte.

### LZ77

LZ77 is more straightforward than Huffman encoding. The worst possible case for time is somewhere under O(*n*\**m*) where *m* is the search length. Space complexity is O(*n*). Memory contains uncompressed data in its entirety, but in addition to that, there are only a few variables.  

## Performance

Results of a benchmark run with 10 passes can be found in `benchmark.ods`.

### Sample Data

An attempt was made to include files from a variety of categories:

- Binary
    - Compressed
        - Lossy
          - `*.jpg`
          - `*.mp3`
          - `*.mp4`
        - Lossless
          - `*.epub`
          - `*.flac`
          - `*.png`
    - Uncompressed
        - Contains obvious structure*
          - `*.blend`
          - `*.mid`
          - `*.stl`
        - Mostly random*
          - `*.bmp`
          - `*.wav`
          - `tiracompress`
- Text based
    - Intended for human consumption
          - `*.txt`
          - `*.scad`
          - `*.rs`
    - Not primarily intended for humans**
          - `*.mtl`
          - `*.obj`
          - `*.svg`

*Based on vibes and looking at them with a hex editor.  
(see for yourself by dropping a file to https://hexed.it/)

### Results

Encoding speed, decoding speed, and relative size are very closely correlated for both algorithms. `salsa.mid` stands out as a very fast to compress file, with a high compression ratio. It's also  one of the smallest of the sample files. The cause might be that the file contains only a few unique byte values in repeated patterns, making the hashmap lookup and pointer search faster. LZ77 encoding is slower than decoding. This makes sense as LZ77 does much work up front, searching for matches. Apart from time spent on tree construction, Huffman seems to perform equally when encoding and decoding.

Already compressed file formats compress poorly, gaining size on average with Huffman coding, and always yielding larger files with LZ77. Structured uncompressed and text based perform the best. LZ77 generates smaller archives on average, but suffers more from bad input data.

Huffman coding encodes faster than LZ77, but in most cases decodes slower. LZ77 compresses structured or predictable data to a smaller size.

## Shortcomings

- The program holds the entire uncompressed file in memory.
- Wikipedia mentioned "using two queues" for constructing a Huffman tree. I forgot to look into it.
- I expect there to be a much more performant way to decode and encode Huffman codes than my hashmap implementation. 
- At first, the LZ implementation used LZSS-style bit flags to determine if the next byte is a pointer or literal. This could've been an interesting thing to compare.

## Large Language Models

None used.

## Sources
- https://en.wikipedia.org/wiki/LZ77_and_LZ78
- https://en.wikipedia.org/wiki/Huffman_coding
- Elegant Compression in Text (The LZ 77 Method) - Computerphile - https://www.youtube.com/watch?v=goOa3DGezUA
- https://en.wikipedia.org/wiki/Lempel%E2%80%93Ziv%E2%80%93Storer%E2%80%93Szymanski