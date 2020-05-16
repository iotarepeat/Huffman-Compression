# What is it
A simple command line utility to compress files

# How to use it
```bash
./rust_compression -c <FILE> # To compress
./rust_compression -d <FILE> # To extract/decompress
./rust_compression --help 	 # Get help
```
# How it works
At it's core compression is done using [Huffman Coding](https://en.wikipedia.org/wiki/Huffman_coding)

# Known limitations
- Only works on files containing printable characters
- It may work on arbitrary files but not guaranteed.
- Files containing null byte \0 will not work
- Files having a lot (more than 128) of unique characters will give unexpected results.
