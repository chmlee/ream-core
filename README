# ream-core

[![license](https://img.shields.io/crates/l/ream)](https://github.com/chmlee/ream-core/blob/master/LICENSE)
[![version](https://img.shields.io/crates/v/ream?style=flat)](https://crates.io/crates/ream)


REAM is a data language for building maintainable social science datasets.
It has three main components:

- a **data serialization language** or structured datasets (working in progress)
- a **data template language** to generate datasets (planned)
- a collection of **filters** to manipulate data (planned)

The language compiles to both human-readable documentation (HTML, PDF, etc.) and analysis-ready datasets (CSV, JSON, etc.)
Two formats, one source.

```md
# Country
- name: Belgium
- capital: Brussels
- population: 11433256
  > data from 2019; retrieved from World Bank
- euro_zone: TRUE
  > joined in 1999

## Language
- name: Dutch
- size: 0.59

## Language
- name: French
- size: 0.4

## Language
- name: German
- size: 0.01
```

The official [REAM documentation](https://ream-lang.org) provides more information on the language.
The rest of the README focuses on the compiler, ream-core.

## Usage

### Web

Two web-based editors with ream-core embedded are available without local installation:

- [ream-yew](https://chmlee.github.io/ream-editor)
- [ream-wasm](https://chmlee.github.io/ream-wasm)

### CLI

For a local copy of the commandline tool, you will need [Cargo](https://doc.rust-lang.org/stable/cargo/) and install in one of the two ways:

1. Download the latest tagged version from [crates.io](https://creates.io/crates/ream):

```shell
cargo install ream
```

2. Compile the latest development version from source:

```shell
git clone https://github.com/chmlee/ream-core
cd ream-core
cargo build
```

Now you have commandline tool `ream` available locally.

To compile your REAM file, execute:

```shell
ream -i <INPUT> -o <OUTPUT> -f <FORMAT> [-p]
```

where `<INPUT>` is the path to the REAM file and `<OUTPUT>` the path of the output file.
For `<FORMAT>` there are two output formats to choose from: `CSV` and `AST`.
If the `-p` flag is present, the output will also be printed out as stdout.

Example:

```shell
ream -i data.ream -o data.csv -f CSV -p
```

### Crate

To include ream-core into your project, add the following line to your `Cargo.toml` file:
```toml
[dependencies]
ream = "0.3.1"
```

See [docs.rs](https://docs.rs/ream/0.3.1/ream/) for more information.
