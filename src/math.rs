//#![no_std]
use agb::fixnum::Num;

pub fn matmul(matrix: [[Num<i32, 8>; 3]; 3], vector: [Num<i32, 8>; 3]) -> [Num<i32, 8>; 3] {
    let mut result: [Num<i32, 8>; 3] = [Num::new(0); 3];

    for i in 0..3 {
        result[i] = matrix[i][0] * vector[0] + matrix[i][1] * vector[1] + matrix[i][2] * vector[2];
    }

    return result;
}

pub fn vector_cross(vec1: [Num<i32, 8>; 3], vec2: [Num<i32, 8>; 3]) -> [Num<i32, 8>; 3] {
    let mut result: [Num<i32, 8>; 3] = [Num::new(0); 3];

    // Cross product formula
    result[0] = vec1[1] * vec2[2] - vec1[2] * vec2[1]; // x component
    result[1] = vec1[2] * vec2[0] - vec1[0] * vec2[2]; // y component
    result[2] = vec1[0] * vec2[1] - vec1[1] * vec2[0]; // z component

    return result;
}

pub fn vector_dot(vec1: [Num<i32, 8>; 3], vec2: [Num<i32, 8>; 3]) -> Num<i32, 8> {
    let mut result: Num<i32, 8> = Num::new(0);

    for i in 0..3 {
        result = result + vec1[i] * vec2[i];
    }

    return result;
}

#[allow(dead_code)]
pub fn vector_add(vec1: [Num<i32, 8>; 3], vec2: [Num<i32, 8>; 3]) -> [Num<i32, 8>; 3] {
    let mut result: [Num<i32, 8>; 3] = [Num::new(0); 3];

    for i in 0..3 {
        result[i] = vec1[i] + vec2[i];
    }

    return result;
}


pub fn vector_sub(vec1: [Num<i32, 8>; 3], vec2: [Num<i32, 8>; 3]) -> [Num<i32, 8>; 3] {
    let mut result: [Num<i32, 8>; 3] = [Num::new(0); 3];

    for i in 0..3 {
        result[i] = vec1[i] - vec2[i];
    }

    return result;
}


pub fn normalize(v: [Num<i32, 8>; 3]) -> [Num<i32, 8>; 3] {
    let length: Num<i32, 8> = fast_sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    //let length: Num<i32, 8> = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / length, v[1] / length, v[2] / length]
}

//crude approximation is => x/2 + 1/2
pub fn fast_sqrt(n: Num<i32, 8>) -> Num<i32, 8> {
    return (n/2) + (Num::new(1) / Num::new(2));
}