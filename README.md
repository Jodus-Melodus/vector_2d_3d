# vector_2d_3d
Simple small library for 2D and 3D vectors


## Vector2D

```rust
#[derive(Debug, Copy, Clone)]
struct Vector2D {
    x: f32,
    y: f32,
}
```

### Functions
#### Constructors
- `from_coord` - creates a vector from the x and y coordinate
- `from_coords` - creates a vector from the two x and y coordinates
- `from_mag_theta` - creates a vector from the given magnitude and degrees (in radians) from the x-axis
#### Methods
- `ihat` - returns the unit vector of the x-axis
- `jhat` - returns the unit vector of the y-axis
- `perpendicular_cw` - returns a vector that is 90 degrees clockwise to the current vector
- `perpendicular_ccw` - returns a vector that is 90 degrees counter clockwise to the current vector
- `magnitude` - returns the magnitude of the current vector
- `direction_as_unit_vector` - returns a unit vector with the same direction of the current vector
- `direction` - returns the degrees (theta) between the vector and the x-axis
- `dot_product` - returns the dot product of the current vector and `other`
- `dot_product_with_angle` - returns the dot product of the current vector and `other` using their magintudes and the angle between them
- `cross_product` - programmers implementation of the cross product between 2D vectors, since it doesn't exist

### Traits

Vector2D implements the `Add<f32>`, `Sub<f32>`, `Mul<f32>`, `Div<f32>` traits.