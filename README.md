# Another Rock Paper Scissors Game (<abbr>ARPSG</abbr>)

Another Rock Paper Scissors Game (<abbr>ARSPG</abbr>) is a simple terminal game
written in Rust.

## Documentation

Following is the documentation for
<abbr title="Another Rock Paper Scissors Game">ARPSG</abbr>
[v0.1.0](https://github.com/acodili-jg/arpsg/release/tag/v0.1.0):

### Gameplay

 1. The program asks the player between `rock`, `paper`, and `scissors`
    (case-insensitive). It will keep asking until a valid option is entered.
 2. The program runs the Random Number Generator (RNG) to choose as well.
 3. The outputs from `1.` and `2.` are compared to determine a win, draw, or
    loss.
 4. The program terminates.

## Building

Checks the project for mistakes with clippy:
```sh
cargo clippy
```

Creates a debug build of the project:
```sh
cargo build
```

Runs tests on the project:
```sh
cargo test
```

Creates and run a debug field of the project:
```sh
cargo run
```

To create and or run a release build, simply add `--release`.
