# Creating new levels

The quickest way to see how levels are structured is to see the contents of the simplest level at `/levels/00.json`

## Project structure

Each level is described as a json array in a separate .json file in the `levels` folder. The files are read by `scripts/bundle_levels.py` and bundled into the program when the project is built.

During the build, levels are sorted into alphabetical order based on their filenames. They will appear in the game menu in that order. For example in the existing set of levels, `levels/00.json` is currently the first option in the level selection menu.

## Level content guidelines

### Level content

Levels are built from blocks aka *entities*. The .json file should contain an array of these entities.

Each level should have at least the following entities.

* A starting platform at xyz coordinates `[0,0,0]`
* A finish entity somewhere in the level, so the level can end.

Entities have properties such as 

* type
* size
* position

Individual entity types and their parameters are described in a separate level specification under the `/docs` folder.

### 3d axes

* x for side to side
* y for up/down, positive is up, negative is down
* z for forward/back

The player will spawn at xyz coordinate `[0,3,0]`, so adding a block at `[0,0,0]` will make sure they have something to stand on when the level start.