#todo: convert to "raw" x << 8 form, so they can be inited with Num::from_raw()
#      later on could premake rotation tables for camera to avoid calculating cos & sin


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
    # Starting at (0, -3) at angle 0
    x = radius * math.sin(angle_radians)  # sin controls x, starts from 0
    y = -radius * math.cos(angle_radians)  # cos controls y, starts from -3

    #print(f"Angle: {i/256:.6f} , X: {x:.6f}, Z: {y:.6f}")


    formatted_x = int(x * 256)
    formatted_y = int(y * 256)
    formatted_angle = int(i)


    formatted_string = f"[Fixed::from_raw({formatted_x}), Fixed::from_raw({formatted_y}), Fixed::from_raw({formatted_angle})],"
    print (formatted_string)