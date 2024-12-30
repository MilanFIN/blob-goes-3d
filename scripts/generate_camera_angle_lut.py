"""
import math

# Parameters
num_points = 256
radius = 5  # Radius of the circle
center_x, center_y = 0, 0  # Center of the circle

# Compute and print results
for i in range(num_points):
    # Calculate the current angle in degrees
    angle_degrees = i * (360 / num_points)
    angle_radians = math.radians(angle_degrees)

    # Calculate x and y coordinates on the edge of the circle
    # Starting at (0, -5) at angle 0
    x = radius * math.sin(angle_radians)
    y = -radius * math.cos(angle_radians)

    formatted_x = int(x * 256)
    formatted_y = int(y * 256)
    formatted_angle = int(i)


    formatted_string = f"[Fixed::from_raw({formatted_x}), Fixed::from_raw({formatted_y}), Fixed::from_raw({formatted_angle})],"
    print (formatted_string)
"""

import math

# Parameters
num_points = 256
radius = 5  # Radius of the circle
center_x, center_z = 0, 0  # Center of the circle in the xz-plane
fixed_tilt_x = -12  # Maximum tilt for x-axis when y angle is 0

# Compute and print results
for i in range(num_points):
    # Calculate the current angle in degrees
    angle_degrees = i * (360 / num_points)
    angle_radians = math.radians(angle_degrees)

    # Calculate x and z coordinates on the edge of the circle
    # Starting at (0, -5) at angle 0
    x = radius * math.sin(angle_radians)
    z = -radius * math.cos(angle_radians)

    # Compute fixed angles for the camera tilt
    x_angle = fixed_tilt_x * math.cos(angle_radians)  # Max range: -12 to 12
    z_angle = fixed_tilt_x * math.sin(angle_radians)  # Max range: -12 to 12

    # Convert to fixed-point format
    formatted_x = int(x * 256)
    formatted_z = int(z * 256)
    formatted_y_angle = int(i)
    formatted_x_angle = int(-12) 
    formatted_z_angle = 0#int(z_angle)  

    # Print the result in the desired format
    formatted_string = (
        f"[Fixed::from_raw({formatted_x}), Fixed::from_raw({formatted_z}), Fixed::from_raw({formatted_y_angle}), "
        f"Fixed::from_raw({formatted_x_angle}), Fixed::from_raw({formatted_z_angle})],"
    )
    print(formatted_string)
