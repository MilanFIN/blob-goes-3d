const LEVEL1: &str = r#"
[
    { "type": "cube", "data": { "size": 3, "x": 0, "y": 0, "z": 0 } },
    { "type": "empty", "data": {} }
]
"#;

const LEVEL2: &str = r#"
[
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 5, "x": 0, "y": -5, "z": 0 } },
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 5, "x": 0, "y": 10.1, "z": 0 } }
]
"#;

//    { "type": "cube", "data": { "size": 2.0, "x": 10, "y": -10, "z":0  } },
//    { "type": "rectangle", "data": { "xsize": 5, "ysize": 0.2, "zsize": 5, "x": 0, "y": -10, "z": 0 } }
//    { "type": "cube", "data": { "size": 1.0, "x": 4.2, "y": 0, "z": 0 } }

// The result is a constant array of string slices.
pub const LEVELS: [&'static str; 2] = [LEVEL1, LEVEL2];
