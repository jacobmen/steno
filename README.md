# steno

## About
This is a basic stenography library I made to learn about Rust and its ecosystem.
It primarily lets you encode text (ASCII only for now) into an image and later decode it.
The encoding algorithm minimizes noticeable change to the image by only modifying the least significant bits
of each pixel's channel values.

## Usage
For now, there are two primary commands (run `steno \<command\> -h` for specifics for each command):
* `encode`: Encodes text into an image
* `decode`: Decodes text from an image and outputs to `stdout`

## Dependencies
[clap](https://crates.io/crates/clap): CLI parsing and command recommendations

[image](https://crates.io/crates/image): Image manipulation

## Installation
1. Install [Rust](https://www.rust-lang.org/) with cargo
2. Clone this repository
3. Run `cargo install` from within the directory. This should install all the dependencies, build steno, and add it to your `$PATH`
