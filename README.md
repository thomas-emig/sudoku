# Sudoku

Sudoku solver/generator as a Rust learning project

I started this project to get to know the programming language Rust. It turned out to be a nice command line tool and library to generate and solve sudoku puzzles.

Just as convolutional codes, the solution to a sudoku puzzle can be modeled as a tree structure. So this library implements a type of stack solver which executes a depth-first-search. In some cases, this might take a long time as especially larger puzzles can have a lot of branches.

## Build and run

- `cargo run --release`

## Command line arguments

```
-s :                  Solve puzzles from stdin. Board size is determined from input string.
-b :    -b <Base>     Set base for puzzle generation (2-9).
-n :    -n <Number>   Generate n puzzles.
-p :                  Pretty print puzzles instead of one-line output.
-h :                  Print help.
```
