# Random Quoter
Is a program that displays random quotes from the dataset given by ADA 2021 EPFL, [here](https://drive.google.com/file/d/1MtCmY5zeLhdKOw8aGCgE_e5yVaODkZYW/view?usp=sharing).

## Installation (basically compile it yourself)

1. Install rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
1. `git clone` this repositiory
1. make sure the dataset is extracted in `../datasets` and not renamed.
1. otherwise you can rename the file path in the line that says "`File::open`" in `src/main.rs`
1. executing `cargo run --release` will build and run the program

## Usage
Entering "q" will terminate the program. Entering any other string will print 10 random quotes. As usual pressing Ctrl-c will also close the program.