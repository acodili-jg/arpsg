# Another Rock Paper Scissors Game (<abbr>ARPSG</abbr>)

Another Rock Paper Scissors Game (<abbr>ARSPG</abbr>) is a simple terminal game
written in Rust.

## Documentation

Following is the documentation for
<abbr title="Another Rock Paper Scissors Game">ARPSG</abbr>
[v0.2.0](https://github.com/acodili-jg/arpsg/releases/tag/v0.2.0):

### Gameplay

 1. The program runs a Random Number Generator (<abbr>RNG</abbr>) to choose between `rock`,
    `paper`, and `scissors`.
 2. The program asks the player to choose too.
 3. The choices made from `1.` and `2.` are then compared to determine if the
    player won or lost; in case of a tie, the program starts back to `1.`.
 4. The program terminates.

## Building

> For details on cloning this repository:    
> https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository
---

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

Creates and run a debug build of the project:
```sh
cargo run
```

To create and or run a release build, simply add `--release`.
