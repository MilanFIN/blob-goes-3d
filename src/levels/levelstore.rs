pub const LEVELSIZE: usize = 30;

const LEVEL1: &str = r#"
[
    { "type": "cube", "data": { "size": 3, "x": 10, "y": 0, "z": 0 } },
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 5, "x": 0.0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "ice", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 3, "y": 1, "z": 0, "color": 2, "acceleration": 0.004, "rotation": 0.0 } }
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

const LEVEL4: &str = r#"
[
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0.0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 4.0, "y": 3.5, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0.0, "y": 7, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 4.0, "y": 10.5, "z": 0, "color": 2, "rotation": 0.0 } },
    {"type": "mover",
        "data": { 
            "xsize": 4, "ysize": 1, "zsize": 4, "x": 8, "y": 10.5, "z": 0,
            "pos_a_x": 8, "pos_a_y": 10.5, "pos_a_z": 0,
            "pos_b_x": 28, "pos_b_y": 15, "pos_b_z": 7,
            "speed": 2, "wait": 20
        }
    },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 32, "y": 15, "z": 7, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 28, "y": 18.5, "z": 7, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 32, "y": 22, "z": 7, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 28, "y": 25.5, "z": 7, "color": 2, "rotation": 0.0 } },
    { "type": "finish", "data": {"size": 3, "x": 28, "y": 28.5, "z": 7, "color": 2, "rotation": -0.22 } }

]
"#;

const LEVEL5: &str = r#"
[
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 2, "x": 0.0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 0, "y": 0, "z": 5, "rotation": 0, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 1, "y": 0, "z": 10, "rotation": 0.1, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 4, "y": 0, "z": 15, "rotation": 0.2, "lifetime": 25 } },
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 3, "x": 10, "y": 0, "z": 17, "color": 2, "rotation": 0.3 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 16, "y": 0, "z": 15, "rotation": 0.4, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 18, "y": 0, "z": 20, "rotation": 0.2, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 21, "y": 0, "z": 25, "rotation": 0.1, "lifetime": 25 } },
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 3, "x": 21, "y": 0, "z": 30, "color": 2, "rotation": 0.0 } },
    { "type": "crumbling", "data": { "xsize": 4, "ysize": 0.5, "zsize": 3, "x": 21, "y": 1, "z": 35, "rotation": 0.0, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 4, "ysize": 0.5, "zsize": 3, "x": 21, "y": 2, "z": 40, "rotation": 0.0, "lifetime": 25 } },
    { "type": "rectangle", "data": { "xsize": 4, "ysize": 1, "zsize": 3, "x": 21, "y": 4, "z": 45, "color": 2, "rotation": 0.0 } },
    { "type": "finish", "data": {"size": 3, "x": 21, "y": 7, "z": 45, "color": 2, "rotation": 0 } }

]
"#;

const LEVEL6: &str = r#"
[
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 0, "y": 0, "z": 5, "rotation": 0, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 0, "y": 0, "z": 10, "rotation": 0, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 0, "y": 0, "z": 15, "rotation": 0, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 0, "y": 0, "z": 20, "rotation": 0, "lifetime": 25 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 25, "color": 2, "rotation": 0.0 } },
    { "type": "switch", "data": {"size": 2, "x": 0, "y": 1, "z": 25.5, "color": 3, "rotation": 0.125 } },
    { "type": "wireframe", "data": {"xsize": 2, "ysize": 1, "zsize": 2, "x": -5, "y": 0, "z": 0, "color": 0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -10, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "finish", "data": {"size": 3, "x": -10, "y": 3, "z": 0, "color": 2, "rotation": 0.25 } }
]
"#;


const LEVEL7: &str = r#"
[
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 3, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 1, "y": 0, "z": 6, "color": 2, "rotation": 0.1 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 9, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 1, "y": 0, "z": 12, "color": 2, "rotation": -0.1 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 15, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 1, "y": 0, "z": 18, "color": 2, "rotation": 0.1 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 21, "color": 2, "rotation": 0.25 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 24, "color": 2, "rotation": 0.25 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 27, "color": 2, "rotation": 0.25 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 30, "color": 2, "rotation": 0.25 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 33, "color": 2, "rotation": 0.25 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 38, "color": 2, "rotation": 0.0 } },
    { "type": "finish", "data": {"size": 3, "x": 2, "y": 3, "z": 38, "color": 2, "rotation": 0 } }
]
"#;

const LEVEL8: &str = r#"
[
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 5,
            "pos_a_x": -5, "pos_a_y": 0, "pos_a_z": 5,
            "pos_b_x": 5, "pos_b_y": 0, "pos_b_z": 5,
            "speed": 0.7, "wait": 3
        }
    },
    {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 10,
            "pos_a_x": 5, "pos_a_y": 0, "pos_a_z": 10,
            "pos_b_x": -5, "pos_b_y": 0, "pos_b_z": 10,
            "speed": 0.7, "wait": 3
        }
    },
    {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 15,
            "pos_a_x": -5, "pos_a_y": 0, "pos_a_z": 15,
            "pos_b_x": 5, "pos_b_y": 0, "pos_b_z": 15,
            "speed": 0.7, "wait": 3
        }
    },
    {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 20,
            "pos_a_x": 5, "pos_a_y": 0, "pos_a_z": 20,
            "pos_b_x": -5, "pos_b_y": 0, "pos_b_z": 20,
            "speed": 0.7, "wait": 3
        }
    },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 25, "color": 2, "rotation": 0.0 } },
       {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 35,
            "pos_a_x": 0, "pos_a_y": 0, "pos_a_z": 30,
            "pos_b_x": 0, "pos_b_y": 0, "pos_b_z": 40,
            "speed": 0.7, "wait": 5
        }
    },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 25, "color": 2, "rotation": 0.0 } },
       {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 50,
            "pos_a_x": 0, "pos_a_y": 0, "pos_a_z": 55,
            "pos_b_x": 0, "pos_b_y": 0, "pos_b_z": 45,
            "speed": 0.7, "wait": 5
        }
    },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 60, "color": 2, "rotation": 0.0 } },

    {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 65,
            "pos_a_x": 0, "pos_a_y": 10, "pos_a_z": 65,
            "pos_b_x": 0, "pos_b_y": -10, "pos_b_z": 65,
            "speed": 0.7, "wait": 3
        }
    },
    {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 70,
            "pos_a_x": 0, "pos_a_y": -10, "pos_a_z": 70,
            "pos_b_x": 0, "pos_b_y": 10, "pos_b_z": 70,
            "speed": 0.7, "wait": 3
        }
    },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 75, "color": 2, "rotation": 0.0 } },
    { "type": "finish", "data": {"size": 3, "x": 0, "y": 3, "z": 75, "color": 2, "rotation": 0 } }
]
"#;

const LEVEL9: &str = r#"
[
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "bounce", "data": { "xsize": 2, "ysize": 1, "zsize": 2, "x": 0, "y": 0, "z": 5, "color": 3, "power": 1.5, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 12, "color": 2, "rotation": 0.0 } },
    { "type": "bounce", "data": { "xsize": 2, "ysize": 1, "zsize": 2, "x": 2, "y": 0, "z": 17, "color": 3, "power": 1.5, "rotation": 0.0 } },
    { "type": "bounce", "data": { "xsize": 2, "ysize": 1, "zsize": 2, "x": -2, "y": 0, "z": 23, "color": 3, "power": 1.5, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 28, "color": 2, "rotation": 0.0 } },
    { "type": "bounce", "data": { "xsize": 2, "ysize": 1, "zsize": 2, "x": 5, "y": 0, "z": 28, "color": 3, "power": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 10, "y": 15, "z": 28, "color": 2, "rotation": 0.0 } },
    { "type": "finish", "data": {"size": 3, "x": 10, "y": 18, "z": 28, "color": 2, "rotation": 0.25 } }
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
pub const LEVELS: [&'static str; 9] = [LEVEL1, LEVEL2, LEVEL3, LEVEL4, LEVEL5, LEVEL6, LEVEL7, LEVEL8, LEVEL9];
