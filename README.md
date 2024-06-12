# PARTITION 

Rust implementation of the _Approximate Partition_ problem. The crate can be used as a rust library or as a standalone CLI. 

## PDF 
The PDF with the description of the alghoritm can be downloaded from the `Releases` section of the repository under `Assets -> main.pdf`. 

[Latest release](https://github.com/piniom/partition/releases/latest)
## Rust

To build, run, and test the project you need to have [rust installed](https://www.rust-lang.org/tools/install).

## CLI

### Running

To __run__ the cli run:
```bash
cargo run --release -- -h
```
This will print the help section of the binary. The double dashes are used to split the binary arguments from `cargo` arguments.

### Building

To __build__ the cli run:
```bash
cargo build --release
```
This will build the executable. The binary will be outputed to `target/release/partition`.

## Building PDF from LaTeX

The source code of the PDF is located in the `paper` folder.

To compile the PDF, `cd` to the `paper` folder:
```bash
cd paper
```
and run
```bash
./compile.sh
```

To automatically re-compile upon file save, run
```bash
./auto_recompile.sh
```
in the ```paper``` directory. Keep this terminal open for the auto re-compile to work.


## Tests

To run the tests run
```bash
cargo run
```
in the root directory.

Optionally you can provide a feature flag to run the property-based tests. Note that this may take very long to complete.

```bash
cargo test --features use-proptest
```

