# vector_2d_3d
- Simple small library for 2D and 3D vectors.
- Please feel free to review and leave comments on my GitHub page.

## Vector2D
```rust
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}
```

### Functions
#### Constructors
- `new` - creates a vector from the x and y coordinate
- `from_coord` - creates a vector from the x and y coordinate
- `from_coords` - creates a vector from the two x and y coordinates
- `from_mag_theta` - creates a vector from the given magnitude and degrees (in radians) from the x-axis
- `ihat` - creates the unit vector of the x-axis
- `jhat` - creates the unit vector of the y-axis
#### Methods
- `perpendicular_cw` - returns a vector that is perpendicular clockwise to the current vector
- `perpendicular_ccw` - returns a vector that is perpendicular counter clockwise to the current vector
- `magnitude` - returns the magnitude of the current vector
- `direction_as_unit_vector` - returns a unit vector with the same direction of the current vector
- `direction` - returns the degrees (theta) between the vector and the x-axis
- `dot_product` - returns the dot product of the current vector and `other`
- `dot_product_with_angle` - returns the dot product of the current vector and `other` using their magintudes and the angle between them
- `cross_product` - programmers implementation of the cross product between 2D vectors, since it doesn't exist

### Traits

Vector2D implements the `Add<f32>`, `Sub<f32>`, `Mul<f32>`, `Div<f32>` traits.

## Vector3D
```rust
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
```
### Functions
#### Constructors
- `new` - creates a vector from the x, y and z coordinate
- `from_coord` - creates a vector from the x, y and z coordinate
- `from_mag_alpha_beta_gamma` - creates a vector from the magnitude and angles between the x, y and z-axes
- `from_mag_asimuthal_polar` - creates a vector from the magnitude and angle azimuthal and polar
- `ihat` - creates the unit vector of the x-axis
- `jhat` - creates the unit vector of the y-axis
- `khat` - creates the unit vector of the z-axis
#### Methods
- `perpendicular` - returns a vector that is perpendicular to the current vector
- `magnitude` - returns the magnitude of the current vector
- `direction_as_unit_vector` - returns a unit vector with the same direction as the current vector
- `direction_from_axes` - returns the angles between the different axes
- `direction` - returns the direction of the current vector
- `dot_product` - returns the dot product of the current vector and `other`
- `cross_product` - returns the cross product of the current vector and `other`
### Traits

Vector3D implements the `Add<f32>`, `Sub<f32>`, `Mul<f32>`, `Div<f32>` traits.