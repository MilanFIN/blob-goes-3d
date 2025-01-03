# WIP GBA 3d platformer

A software renderer and a platformer for the GBA. Very much WIP and not yet playable.

## Building

### Prerequisites

* [rustup](https://www.rust-lang.org/tools/install)
* A gba emulator (mgba is included as a dependency)
* gbafix to generate the .gba rom file (agb-gbafix included as a dependency)

### Running in an emulator

```sh
cargo build --release
```

```sh
cargo run --release
```

### Generaging .gba rom

```sh
agb-gbafix target/thumbv4t-none-eabi/release/<binary name> -o <game name>.gba
```

## Features

- [x] A somewhat functional software 3d renderer (MODE 4)
- [x] Barebones implementation for loading levels from json strings
- [x] Basic level building blocks
- [x] Basic collision detection

## TODO

- [ ] Color options and shading based on polygon face angles
- [ ] Bug free collision detection with sliding against walls
- [ ] A simple and documented method for defining new levels as .json files
- [ ] Actual platformer features (collapsing/moving platforms etc., finishing a level)
- [ ] Lots of optimizations
