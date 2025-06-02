use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2D { x, y }
    }
    pub fn ihat() -> Self {
        Vector2D { x: 1.0, y: 0.0 }
    }

    pub fn jhat() -> Self {
        Vector2D { x: 0.0, y: 1.0 }
    }

    pub fn perpendicular_ccw(&self) -> Vector2D {
        Vector2D {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn perpendicular_cw(&self) -> Vector2D {
        Vector2D {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn direction_as_unit_vector(&self) -> Option<Vector2D> {
        let mag = self.magnitude();
        if mag == 0.0 {
            None
        } else {
            Some(Vector2D {
                x: self.x / mag,
                y: self.y / mag,
            })
        }
    }

    pub fn direction(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn dot_product(&self, other: &Vector2D) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn dot_product_with_angle(&self, other: &Vector2D, angle: f32) -> f32 {
        self.magnitude() * other.magnitude() * angle.cos()
    }

    pub fn cross_product(&self, other: &Vector2D) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl Mul<f32> for Vector2D {
    type Output = Vector2D;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vector2D {
    type Output = Vector2D;
    fn div(self, rhs: f32) -> Self::Output {
        Vector2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Add for Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3D { x, y, z }
    }

    pub fn ihat() -> Self {
        Vector3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn jhat() -> Self {
        Vector3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub fn khat() -> Self {
        Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    pub fn perpendicular(&self) -> Vector3D {
        let other = if self.x == 0.0 && self.y == 0.0 {
            Vector3D::jhat()
        } else {
            Vector3D::ihat()
        };
        self.cross_product(&other)
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn direction_as_unit_vector(&self) -> Vector3D {
        Vector3D {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
        }
    }

    pub fn direction_from_axes(&self) -> Vector3D {
        let mag = self.magnitude();
        Vector3D {
            x: (self.x / mag).acos(),
            y: (self.y / mag).acos(),
            z: (self.z / mag).acos(),
        }
    }

    pub fn direction(&self, other: &Vector3D) -> f32 {
        let dot = self.dot_product(other);
        let mags = self.magnitude() * other.magnitude();
        (dot / mags).acos()
    }

    pub fn dot_product(&self, other: &Vector3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Mul<f32> for Vector3D {
    type Output = Vector3D;
    fn mul(self, rhs: f32) -> Vector3D {
        Vector3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f32> for Vector3D {
    type Output = Vector3D;
    fn div(self, rhs: f32) -> Self::Output {
        Vector3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Add for Vector3D {
    type Output = Vector3D;
    fn add(self, rhs: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;
    fn sub(self, rhs: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2d_mul_div() {
        let v = Vector2D { x: 2.0, y: 4.0 };
        let v2 = v * 2.0;
        assert_eq!(v2.x, 4.0);
        assert_eq!(v2.y, 8.0);

        let v3 = v2 / 2.0;
        assert_eq!(v3.x, 2.0);
        assert_eq!(v3.y, 4.0);
    }

    #[test]
    fn test_vector2d_perpendicular() {
        let v = Vector2D { x: 1.0, y: 2.0 };
        let ccw = v.perpendicular_ccw();
        assert_eq!(ccw.x, -2.0);
        assert_eq!(ccw.y, 1.0);

        let cw = v.perpendicular_cw();
        assert_eq!(cw.x, 2.0);
        assert_eq!(cw.y, -1.0);
    }

    #[test]
    fn test_vector2d_magnitude() {
        let v = Vector2D { x: 3.0, y: 4.0 };
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_vector2d_dot_cross() {
        let v1 = Vector2D { x: 1.0, y: 2.0 };
        let v2 = Vector2D { x: 3.0, y: 4.0 };
        assert_eq!(v1.dot_product(&v2), 11.0);
        assert_eq!(v1.cross_product(&v2), -2.0);
    }

    #[test]
    fn test_vector3d_mul_div() {
        let v = Vector3D {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        };
        let v2 = v * 2.0;
        assert_eq!(v2.x, 4.0);
        assert_eq!(v2.y, 8.0);
        assert_eq!(v2.z, 12.0);

        let v3 = v2 / 2.0;
        assert_eq!(v3.x, 2.0);
        assert_eq!(v3.y, 4.0);
        assert_eq!(v3.z, 6.0);
    }

    #[test]
    fn test_vector3d_magnitude() {
        let v = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 2.0,
        };
        assert_eq!(v.magnitude(), 3.0);
    }

    #[test]
    fn test_vector3d_dot_cross() {
        let v1 = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vector3D {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        assert_eq!(v1.dot_product(&v2), 32.0);
        let cross = v1.cross_product(&v2);
        assert_eq!(cross.x, -3.0);
        assert_eq!(cross.y, 6.0);
        assert_eq!(cross.z, -3.0);
    }

    #[test]
    fn test_vector3d_perpendicular() {
        let v = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let perp = v.perpendicular();
        // Should be perpendicular to (0,0,1), e.g. (1,0,0)
        assert!(v.dot_product(&perp).abs() < 1e-6);
    }
}
