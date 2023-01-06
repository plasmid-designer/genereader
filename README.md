# genereader
> A library to read, manipulate and write various genetic sequencing formats.

Developed for use in [Plasmid Designer].

## About

Genereader provides a universal interface for reading, manipulating and writing various genetic sequencing formats for use in [Plasmid Designer] and similar bioinformatics applications.

**Intended use-case**
1. Read genetic sequences and associated metadata from a file
2. Manipulate the contents of the file using a high-level interface
3. Write the modified data back to disk

Genereader tries to preserve the original formatting as well as _all_ metadata (such as annotations, etc.), but no explicit guarantees are made. Use at your own risk.

## Roadmap

| Format             | Status |
| ------------------ | ------ |
| FASTA              | ✅     |
| FASTQ              | TODO   |
| GB (GenBank)       | TODO   |
| SAM                | TODO   |
| Universal Inteface | TODO   |

## Contributing

Contributions are welcome! Please open an issue or pull request if you have any suggestions or bug reports.

<!-- Links -->

[Plasmid Designer]: https://github.com/plasmid-designer/plasmid-designer