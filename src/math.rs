//#![no_std]
use agb::fixnum::Num;

pub fn matmul(matrix: [[Num<i32, 8>; 3]; 3], vector: [Num<i32, 8>; 3]) -> [Num<i32, 8>; 3] {
    let mut result: [Num<i32, 8>; 3] = [Num::new(0); 3];

    for i in 0..3 {
        result[i] = matrix[i][0] * vector[0] + matrix[i][1] * vector[1] + matrix[i][2] * vector[2];
    }

    return result;
}

pub fn vectorCross(vec1: [Num<i32, 8>; 3], vec2: [Num<i32, 8>; 3]) -> [Num<i32, 8>; 3] {
    let mut result: [Num<i32, 8>; 3] = [Num::new(0); 3];

    // Cross product formula
    result[0] = vec1[1] * vec2[2] - vec1[2] * vec2[1]; // x component
    result[1] = vec1[2] * vec2[0] - vec1[0] * vec2[2]; // y component
    result[2] = vec1[0] * vec2[1] - vec1[1] * vec2[0]; // z component

    return result;
}

pub fn vectorDot(vec1: [Num<i32, 8>; 3], vec2: [Num<i32, 8>; 3]) -> Num<i32, 8> {
    let mut result: Num<i32, 8> = Num::new(0);

    for i in 0..3 {
        result = result + vec1[i] * vec2[i];
    }

    return result;
}

pub fn vectorAdd(vec1: [Num<i32, 8>; 3], vec2: [Num<i32, 8>; 3]) -> [Num<i32, 8>; 3] {
    let mut result: [Num<i32, 8>; 3] = [Num::new(0); 3];

    for i in 0..3 {
        result[i] = vec1[i] + vec2[i];
    }

    return result;
}

pub fn vectorSub(vec1: [Num<i32, 8>; 3], vec2: [Num<i32, 8>; 3]) -> [Num<i32, 8>; 3] {
    let mut result: [Num<i32, 8>; 3] = [Num::new(0); 3];

    for i in 0..3 {
        result[i] = vec1[i] - vec2[i];
    }

    return result;
}
