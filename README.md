# grep-lite

👋🏼 Hi, during my Rust learning journey I'm trying to publish all my exercises.

`grep-lite` is a little grep emulation.

## Install

Requirements:
- rust
- cargo

```bash
$ cargo build
```

## Run

### Find a pattern in a file
```bash
$ cargo run <pattern> <path>
```

Example - find `rust` patter in the `README.md` file:
```bash
$ cargo run rust README.md
```

### Find a pattern from stdin
```bash
$ cargo run <pattern>
```

Example -
```bash
$ cargo run rust 
```
