import math

def generate_rust_sin_array():
    sin_values = [round(math.sin(math.radians(angle)) * 256) for angle in range(90)]
    rust_array = ", ".join(f"Fixed::from_raw({value})" for value in sin_values)
    rust_code = f"const PARTIAL_SINE_LUT: [Fixed; 90] = [ {rust_array} ];"
    return rust_code

if __name__ == "__main__":
    rust_sin_array = generate_rust_sin_array()
    print(rust_sin_array)