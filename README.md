# Rippedgrep

A basic implementation of [Ripgrep](https://github.com/BurntSushi/ripgrep), with the following features:
- Recursive searchign
- Multi-threaded operations
- Flags for different search criteria
- Regex support

### Example
```cargo run -- how poem.txt```

### Build
Build with cargo:
```cargo build```

### Program structure
```
.
├── Cargo.lock
├── Cargo.toml
├── poem.txt
├── src
│   └── main.rs
└── test
    ├── a
    │   ├── hi.txt
    │   └── test.c
    ├── b
    │   └── bye.txt
    └── c
        └── spooky.tmp       # Create and use to test the file ignore
```

