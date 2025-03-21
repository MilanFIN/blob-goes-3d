# WIP GBA 3d platformer

A 3d platformer / engine for the gameboy advance.

## Gameplay Demo

<video width="600" controls>
	<source src="https://github.com/MilanFIN/unnamed-gba-3d/raw/refs/heads/promo/screenshots/gameplay.mp4" type="video/mp4">
	A sample gameplay video.
</video>

## Building

### Prerequisites

* [rustup](https://www.rust-lang.org/tools/install)
* Python 3.x

### Running in an emulator

```sh
cargo run --release
```

### Building the .gba rom

#### Native

```sh
cargo install agb-gbafix

cargo build --release

agb-gbafix target/thumbv4t-none-eabi/release/wip-gba-3d -o wip-gba-3d.gba
```
#### In docker

```sh
docker build -t gameboy-3d .

docker create --name temp-container gameboy-3d

docker cp temp-container:/app/wip-gba-3d.gba .

docker rm temp-container
```


## Features

- A flat shaded software 3d renderer (MODE4)
- Level creation system utilizing json files and bundling them into the rom file at build time.
- Various common platformer game features (moving blocks etc.)
- Basic 3d Collision detection

## Creating new levels

Levels are bundled automatically from the `/levels` folder. See `/docs` folder for details on creating new levels.