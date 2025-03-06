# Level specification

This file describes the parameters of each entity type implemented in the game. Look at the existing level .json files for examples of how they are used.

The parameters should go under the `data` key in the entity json.

xyz coordinates describe the center of an entity.
Colors should match the examples, so the levels remain coherent.

## rectangle

A basic platform with a rectangular collision and no special effects on the player.

### Example

```json
{
    "type": "rectangle",
    "data": {
        "xsize": 5,
        "ysize": 1,
        "zsize": 5,
        "x": 0.0,
        "y": 0,
        "z": 0,
        "color": 6,
        "rotation": 0.0
    }
}
```

### Parameters

| Parameter | Type  | Description                                                              |
| --------- | ----- | ------------------------------------------------------------------------ |
| xsize     | float | Size of the rectangle along the x-axis                                   |
| ysize     | float | Size of the rectangle along the y-axis                                   |
| zsize     | float | Size of the rectangle along the z-axis                                   |
| x         | float | Position of the rectangle along the x-axis                               |
| y         | float | Position of the rectangle along the y-axis                               |
| z         | float | Position of the rectangle along the z-axis                               |
| color     | u16   | Color of the rectangle (palette index)                                   |
| rotation  | float | Rotation of the entity. Normalized between [0, 1], negative wraps around |

## Finish

The finish portal, which the player must reach. Usually set to +3 in the y direction above the last platform in the game.

### Example

```json
{
    "type": "finish",
    "data": {
        "size": 3.0,
        "x": -19,
        "y": 3,
        "z": 19,
        "color": 2,
        "rotation": 0.0
    }
}
```

### Parameters

| Parameter | Type  | Description                                                              |
| --------- | ----- | ------------------------------------------------------------------------ |
| size      | float | Size of the finish portal                                                |
| x         | float | Position of the finish portal along the x-axis                           |
| y         | float | Position of the finish portal along the y-axis                           |
| z         | float | Position of the finish portal along the z-axis                           |
| color     | u16   | Color of the finish portal (palette index)                               |
| rotation  | float | Rotation of the entity. Normalized between [0, 1], negative wraps around |

## Mover

A platform that moves between two points, `pos_a` and `pos_b`, with a specified speed and wait time.

### Example

```json
{
    "type": "mover",
    "data": {
        "xsize": 4,
        "ysize": 1,
        "zsize": 4,
        "x": 8,
        "y": 10.5,
        "z": 0,
        "pos_a_x": 8,
        "pos_a_y": 10.5,
        "pos_a_z": 0,
        "pos_b_x": 28,
        "pos_b_y": 15,
        "pos_b_z": 7,
        "speed": 2,
        "wait": 20,
        "color": 9
    }
}
```

### Parameters

| Parameter | Type  | Description                                                   |
| --------- | ----- | ------------------------------------------------------------- |
| xsize     | float | Size of the mover along the x-axis                            |
| ysize     | float | Size of the mover along the y-axis                            |
| zsize     | float | Size of the mover along the z-axis                            |
| x         | float | Initial position of the mover along the x-axis                |
| y         | float | Initial position of the mover along the y-axis                |
| z         | float | Initial position of the mover along the z-axis                |
| pos_a_x   | float | Position A of the mover along the x-axis                      |
| pos_a_y   | float | Position A of the mover along the y-axis                      |
| pos_a_z   | float | Position A of the mover along the z-axis                      |
| pos_b_x   | float | Position B of the mover along the x-axis                      |
| pos_b_y   | float | Position B of the mover along the y-axis                      |
| pos_b_z   | float | Position B of the mover along the z-axis                      |
| speed     | float | Speed of the mover                                            |
| wait      | int   | Wait time after reaching position A or B, before moving again |
| color     | u16   | Color of the mover (palette index)                            |

## Crumbling

A platform that crumbles and disappears after a specified lifetime when the player steps on it.

### Example

```json
{
    "type": "crumbling",
    "data": {
        "xsize": 4,
        "ysize": 0.5,
        "zsize": 3,
        "x": 21,
        "y": 1,
        "z": 35,
        "rotation": 0.0,
        "lifetime": 25,
        "color": 7
    }
}
```

### Parameters

| Parameter | Type  | Description                                                              |
| --------- | ----- | ------------------------------------------------------------------------ |
| xsize     | float | Size of the crumbling platform along the x-axis                          |
| ysize     | float | Size of the crumbling platform along the y-axis                          |
| zsize     | float | Size of the crumbling platform along the z-axis                          |
| x         | float | Position of the crumbling platform along the x-axis                      |
| y         | float | Position of the crumbling platform along the y-axis                      |
| z         | float | Position of the crumbling platform along the z-axis                      |
| rotation  | float | Rotation of the entity. Normalized between [0, 1], negative wraps around |
| lifetime  | u16   | Amount of in game frames, after which the platform disappears            |
| color     | u16   | Color of the crumbling platform (palette index)                          |

## Wireframe

A platform that might appear as transparent or solid and can be switched between these states with a switch. Transparent platforms have no collision.

### Example

```json
{
    "type": "wireframe",
    "data": {
        "xsize": 2,
        "ysize": 1,
        "zsize": 2,
        "x": -5,
        "y": 0,
        "z": 0,
        "color": 0,
        "solid": false
    }
}
```

### Parameters

| Parameter | Type  | Description                                |
| --------- | ----- | ------------------------------------------ |
| xsize     | float | Size of the wireframe along the x-axis     |
| ysize     | float | Size of the wireframe along the y-axis     |
| zsize     | float | Size of the wireframe along the z-axis     |
| x         | float | Position of the wireframe along the x-axis |
| y         | float | Position of the wireframe along the y-axis |
| z         | float | Position of the wireframe along the z-axis |
| color     | u16   | Color of the wireframe (palette index)     |
| solid     | bool  | Initial state of the platform              |

## Switch

A switch that can be used to toggle the state of wireframe platforms.

### Example

```json
{
    "type": "switch",
    "data": {
        "size": 2,
        "x": 0,
        "y": 1,
        "z": 25.5,
        "color": 3,
        "rotation": 0.125
    }
}
```

### Parameters

| Parameter | Type  | Description                                                              |
| --------- | ----- | ------------------------------------------------------------------------ |
| size      | float | Size of the switch                                                       |
| x         | float | Position of the switch along the x-axis                                  |
| y         | float | Position of the switch along the y-axis                                  |
| z         | float | Position of the switch along the z-axis                                  |
| color     | u16   | Color of the switch (palette index)                                      |
| rotation  | float | Rotation of the entity. Normalized between [0, 1], negative wraps around |

## Bounce

A platform that propels the player upwards when stepped on, with a specified power.

### Example

```json
{
    "type": "bounce",
    "data": {
        "size": 2,
        "height": 1,
        "x": 0,
        "y": 1,
        "z": 4,
        "color": 4,
        "power": 1.5,
        "rotation": 0.0
    }
}
```

### Parameters

| Parameter | Type  | Description                                                                    |
| --------- | ----- | ------------------------------------------------------------------------------ |
| size      | float | Size of the bounce platform                                                    |
| height    | float | Height of the bounce platform                                                  |
| x         | float | Position of the bounce platform along the x-axis                               |
| y         | float | Position of the bounce platform along the y-axis                               |
| z         | float | Position of the bounce platform along the z-axis                               |
| color     | u16   | Color of the bounce platform (palette index)                                   |
| power     | float | Power of the bounce effect, 1 equals regular jump power, 1.5 is a a nice boost |
| rotation  | float | Rotation of the entity. Normalized between [0, 1], negative wraps around       |


## Ice

A slippery platform that affects the player's movement by diminishing the acceleration of the player.

### Example

```json
{
	"type": "ice",
	"data": {
		"xsize": 3,
		"ysize": 1,
		"zsize": 3,
		"x": 3,
		"y": 1,
		"z": 0,
		"color": 8,
		"acceleration": 0.004,
		"rotation": 0.0
	}
}
```

### Parameters

| Parameter    | Type  | Description                                                              |
| ------------ | ----- | ------------------------------------------------------------------------ |
| xsize        | float | Size of the ice platform along the x-axis                                |
| ysize        | float | Size of the ice platform along the y-axis                                |
| zsize        | float | Size of the ice platform along the z-axis                                |
| x            | float | Position of the ice platform along the x-axis                            |
| y            | float | Position of the ice platform along the y-axis                            |
| z            | float | Position of the ice platform along the z-axis                            |
| color        | u16   | Color of the ice platform (palette index)                                |
| acceleration | float | Acceleration effect on the player when moving on the ice platform. Set to 0 to disable the ability to change direction. 0.004 is used in most of the game so far.        |
| rotation     | float | Rotation of the entity. Normalized between [0, 1], negative wraps around |