# WIP GBA 3d platformer

A software renderer and a platformer for the GBA. Very much WIP and not yet playable.

## Building

### Prerequisites

* [rustup](https://www.rust-lang.org/tools/install)
* Python 3.x

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

- [x] A flat shaded software 3d renderer (MODE4)
- [x] Bundling game levels from json at build time and loading them with serde
- [x] Various level building blocks
- [x] Collision detection


## TODO

- [ ] Docs for creating new levels
- [ ] Menus
