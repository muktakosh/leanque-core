# Contributing to leanque-core

## Public domain declaration

`leanque-core` is free and unencumbered software released into the public
domain. We can only accept your contributions if you dedicate it to
the public domain as per the clauses of the [LICENSE](LICENSE). We
request you to please sign the declaration mentioned in
[CREDITS](CREDITS) by adding your name and email to the list of
contributors as part of your patch.

Please refrain from contributing patches that conflict with the
LICENSE or that you do not own the right to dedicate to public domain.

## Build Instructions

### Pre-requisites

- Install [Rust](https://www.rust-lang.org/) `stable` (v1.16+). We recommend using [`rustup`](https://www.rustup.rs/).
- Clone the repo.

### Trigger build

To build the project in `debug` mode, run:

```
$ cargo build
```

### Creating a release build

If you want a build with all optimizations in place, run this at the root of the repo:

```
$ cargo build --release
```

## Tests

To execute tests, run:

```
$ cargo test
```

## API Documentation

To generate API documentation, run:

```
$ cargo doc
```
