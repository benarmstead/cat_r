# cat_r

![workflow](https://github.com/benarmstead/cat_r/actions/workflows/rust.yml/badge.svg)
![workflow](https://github.com/benarmstead/cat_r/actions/workflows/devskim-analysis.yml/badge.svg)

An implementation of cat, written in Rust, and done recursively.

I made this program due to a need to recursively cat through a directory, and its subdirectories.
cat_r is an extreamly quick way to do this, as it is written in rust and highly effecient.

## Compiling from source

`git clone https://github.com/benarmstead/cat_r.git`

`cd cat_r`

`cargo build --release`

### Running

`./target/release/cat_r <directory to cat_r here>`

(The binary can be moved to and run from anywhere)

## Current usage

`catr`: cat's every file in your current directory and subdirectories

`catr <directory here>`
example: `catr ~/a/`: cats every file in the directory and subdirectory of the specified directory (in this case ~/a/)

## Current problems

Not every file can be catted. e.g. Binary files which are executables. The files must be in text format.

## Screenshots

![image](https://user-images.githubusercontent.com/70973680/127119281-e906bef2-34a9-4fb6-9056-5f5ff331428c.png)
