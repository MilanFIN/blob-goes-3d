pub const LEVELSIZE: usize = 30;

const LEVEL1: &str = r#"
[
    { "type": "cube", "data": { "size": 3, "x": 0, "y": 0, "z": 0 } }
]
"#;

const LEVEL2: &str = r#"
[
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 5, "x": 0, "y": -5, "z": 0, "color": 2 } },
    { "type": "crumbling", "data": { "xsize": 5, "ysize": 1, "zsize": 5, "x": 0, "y": -3.5, "z": 3, "rotation": -0.1, "lifetime": 1 } },
    {
        "type": "mover",
        "data": { "xsize": 5, "ysize": 1, "zsize": 5, "x": 0, "y": -3.5, "z": -5,
                "pos_a_x": 0, "pos_a_y": -3.5, "pos_a_z": -5,
                "pos_b_x": 10, "pos_b_y": -3.5, "pos_b_z": -5,
                "speed": 1, "wait": 20
                }
    },
    { "type": "finish", "data": {"size": 2, "x": 0, "y": -3, "z": 2, "color": 2 } },
    { "type": "switch", "data": {"size": 2, "x": 0, "y": -3.5, "z": 1, "color": 3, "rotation": 0.0 } },
    { "type": "wireframe", "data": {"xsize": 5, "ysize": 1, "zsize": 5, "x": 3, "y": -5, "z": 3, "color": 0 } }

] 
"#;

const LEVEL3: &str = r#"
[
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0.0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -0.65, "y": 0, "z": 5, "color": 2, "rotation": -0.04 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -2.56, "y": 0, "z": 9.5, "color": 2, "rotation": -0.08 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -5.6, "y": 0, "z": 13.5, "color": 2, "rotation": -0.12 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -9.5, "y": 0, "z": 16.5, "color": 2, "rotation": -0.16 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -14.5, "y": 0, "z": 18.5, "color": 2, "rotation": -0.18 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -19, "y": 0, "z": 19, "color": 2, "rotation": -0.22 } },
    { "type": "finish", "data": {"size": 3, "x": -19, "y": 3, "z": 19, "color": 2, "rotation": -0.22 } }

]
"#;

/*

(0.00,0.00)
(4.94,0.65)
(9.55,2.56)
(13.50,5.59)
(16.54,9.55)
(18.45,14.16)
(19.10,19.10)
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 2.08, "y": 0, "z": -4.58, "color": 2, "rotation": -0.04 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 3.86, "y": 0, "z": -3.21, "color": 2, "rotation": -0.08 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 5.0, "y": 0, "z": 0.0, "color": 2, "rotation": -0.12 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 3.86, "y": 0, "z": 3.21, "color": 2, "rotation": -0.16 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 2.08, "y": 0, "z": 4.58, "color": 2, "rotation": -0.21 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0.0, "y": 0, "z": 5.0, "color": 2, "rotation": -0.25 } }

*/
/*
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 5, "x": 0, "y": -3.5, "z": -5 } }
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 5, "x": 0, "y": -3.5, "z": -10 } }
*/

//    { "type": "cube", "data": { "size": 2.0, "x": 10, "y": -10, "z":0  } },
//    { "type": "rectangle", "data": { "xsize": 5, "ysize": 0.2, "zsize": 5, "x": 0, "y": -10, "z": 0 } }
//    { "type": "cube", "data": { "size": 1.0, "x": 4.2, "y": 0, "z": 0 } }

// The result is a constant array of string slices.
pub const LEVELS: [&'static str; 3] = [LEVEL1, LEVEL2, LEVEL3];
