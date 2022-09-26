# Split

A simple and very fast implementation of Unix CLI Split tool using Rust,<br />
unlike Unix split, this tool can run on every machine (Windows, Macos, Linux).

### Building

1. cargo build --release

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
> ./split.exe 560mb.csv -l 20000
File:560mb_1.csv written successfully
File:560mb_2.csv written successfully
File:560mb_3.csv written successfully
File:560mb_4.csv written successfully
Finished, time took: 808 milliseconds
```

### Development

#### Todos in the near future

- Add tests
- Split by number of output files

License
----

MIT
