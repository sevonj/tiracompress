# Testing

Instructions for how to run the tests are in the readme.

## What Was Tested?

Every struct within the project is tested in some manner.

All data conversion is primarily tested by cycling conversions back and forth. The automatic tests will cycle both the creation and extraction off `HuffmanArchive` and `LzArchive`, as well as their individual components. Data compression and decompression is also tested .

The main function and the user interface are not tested. Some obvious functions, such as getters are also not tested (at least directly).

## Sample Data

An attempt was made to include files from a variety of categories:

- Binary
    - Compressed
        - Lossy
        - Lossless
    - Uncompressed
        - Contains obvious structure*
        - Mostly random*
- Text based
    - Intended for human consumption
    - Not primarily intended for humans**

*Based on vibes and looking at them with a hex editor.  
(see for yourself by dropping a file to https://hexed.it/)

**Even though it may be considered humand readable to some.

TODO: Write down file types' categories.

## Coverage

Coverage report from friday, 2025-10-03:

| Filename                      | Regions | Missed Regions | Cover   | Functions | Missed Functions | Executed | Lines | Missed Lines | Cover   | Branches | Missed Branches | Cover |
| ----------------------------- | ------- | -------------- | ------- | --------- | ---------------- | -------- | ----- | ------------ | ------- | -------- | --------------- | ----- |
| compressor/bit_io.rs          | 237     | 2              | 99.16%  | 10        | 0                | 100.00%  | 139   | 0            | 100.00% | 0        | 0               | -     |
| compressor/huffman_archive.rs | 248     | 11             | 95.56%  | 9         | 1                | 88.89%   | 104   | 3            | 97.12%  | 0        | 0               | -     |
| compressor/huffman_code.rs    | 60      | 0              | 100.00% | 6         | 0                | 100.00%  | 38    | 0            | 100.00% | 0        | 0               | -     |
| compressor/huffman_io.rs      | 556     | 2              | 99.64%  | 16        | 0                | 100.00%  | 264   | 0            | 100.00% | 0        | 0               | -     |
| compressor/huffman_tree.rs    | 563     | 16             | 97.16%  | 50        | 1                | 98.00%   | 261   | 1            | 99.62%  | 0        | 0               | -     |
| compressor/lz_archive.rs      | 129     | 11             | 91.47%  | 7         | 1                | 85.71%   | 59    | 3            | 94.92%  | 0        | 0               | -     |
| compressor/lz_pointer.rs      | 272     | 4              | 98.53%  | 16        | 0                | 100.00%  | 140   | 0            | 100.00% | 0        | 0               | -     |
| main.rs                       | 252     | 252            | 0.00%   | 7         | 7                | 0.00%    | 139   | 139          | 0.00%   | 0        | 0               | -     |
| TOTAL                         | 2317    | 298            | 87.14%  | 121       | 10               | 91.74%   | 1144  | 146          | 87.24%  | 0        | 0               | -     |

Branch coverage is currently missing, see https://github.com/taiki-e/cargo-llvm-cov/issues/8. I will attempt to produce numbers later.
