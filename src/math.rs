//#![no_std]

use crate::Fixed;

pub fn matmul(matrix: [[Fixed; 3]; 3], vector: [Fixed; 3]) -> [Fixed; 3] {
    let mut result: [Fixed; 3] = [Fixed::const_new(0); 3];

    for i in 0..3 {
        result[i] = matrix[i][0] * vector[0] + matrix[i][1] * vector[1] + matrix[i][2] * vector[2];
    }

    return result;
}

pub fn vector_cross(vec1: [Fixed; 3], vec2: [Fixed; 3]) -> [Fixed; 3] {
    let mut result: [Fixed; 3] = [Fixed::const_new(0); 3];

    // Cross product formula
    result[0] = vec1[1] * vec2[2] - vec1[2] * vec2[1]; // x component
    result[1] = vec1[2] * vec2[0] - vec1[0] * vec2[2]; // y component
    result[2] = vec1[0] * vec2[1] - vec1[1] * vec2[0]; // z component

    return result;
}

pub fn vector_dot(vec1: [Fixed; 3], vec2: [Fixed; 3]) -> Fixed {
    let mut result: Fixed = Fixed::const_new(0);

    for i in 0..3 {
        result = result + vec1[i] * vec2[i];
    }

    return result;
}

#[allow(dead_code)]
pub fn vector_add(vec1: [Fixed; 3], vec2: [Fixed; 3]) -> [Fixed; 3] {
    let mut result: [Fixed; 3] = [Fixed::const_new(0); 3];

    for i in 0..3 {
        result[i] = vec1[i] + vec2[i];
    }

    return result;
}


pub fn vector_sub(vec1: [Fixed; 3], vec2: [Fixed; 3]) -> [Fixed; 3] {
    let mut result: [Fixed; 3] = [Fixed::const_new(0); 3];

    for i in 0..3 {
        result[i] = vec1[i] - vec2[i];
    }

    return result;
}


pub fn normalize(v: [Fixed; 3]) -> [Fixed; 3] {
    let length: Fixed = fast_sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    //let length: Fixed = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / length, v[1] / length, v[2] / length]
}

//crude approximation is => x/2 + 1/2
pub fn fast_sqrt(n: Fixed) -> Fixed {
    return (n/2) + (Fixed::const_new(1) / Fixed::const_new(2));
}