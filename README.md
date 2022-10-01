# Split <img src="https://www.pngrepo.com/png/180449/512/scissors-cut.png" width="35">

A simple and very fast implementation of Unix CLI Split tool using Rust,<br />
unlike Unix split, this tool can run on every machine (Windows, Macos, Linux).

### Installation

1. Download and install ./bin/split-0.1.0-x86_64.msi

### Usage:

```

split.exe [OPTIONS] <PATH>

ARGS:
    <PATH>    The path to the file to read

OPTIONS:
    -h, --help                  Print help information
        --ignore-empty-lines    Ignore empty lines,
    -l, --lines <LINES>         Number of lines per output file [default: 1000]
    -V, --version               Print version information

```

### Example:

```
> split 560mb.csv -l 20000 --ignore-empty-lines 
File:560mb_1.csv written successfully
File:560mb_2.csv written successfully
File:560mb_3.csv written successfully
File:560mb_4.csv written successfully
Finished, time took: 767 milliseconds
```
### Building

1. cargo build --release

### Development

#### Todos in the near future

- ~~Add tests~~
- Split by number of output files

License
----

MIT

Icons by svgrepo.com