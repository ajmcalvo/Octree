/*
 * File: point3d.rs
 * Created Date: Thursday August 25th 2022
 * Author: Adrian J. Montero Calvo
 * -----
 * Last Modified:
 * Modified By:
 * -----
 * 2022 Adrian J. Montero Calvo
 * License MIT
 */

/// A 3-D point, with x, y, and z fields.
#[derive(Default, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub res: u8,
}

impl Point3D {
    /// Creates a new Point3D,
    pub fn new(x: f64, y: f64, z: f64, res: u8) -> Self {
        let mut p3_d = Self {
            x: x,
            y: y,
            z: z,
            res,
        };
        p3_d.round();
        p3_d
    }

    pub fn round(&mut self) {
        let factor: f64 = 10_f64.powi(self.res as i32);

        let x = (&self.x * factor).round() / factor;
        let y = (&self.y * factor).round() / factor;
        let z = (&self.z * factor).round() / factor;

        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn middle(&self, other: &Self) -> Self {
        let mut p3_d = Point3D {
            x: (self.x + other.x) * 0.50_f64,
            y: (self.y + other.y) * 0.50_f64,
            z: (self.z + other.z) * 0.50_f64,
            res: self.res,
        };
        p3_d.round();
        p3_d
    }

    pub fn rel_position(&self, other: &Self) -> usize {
        let mut result = 0x0 as usize;
        if self.x >= other.x {
            result = result | 0b100;
        }
        if self.y >= other.y {
            result = result | 0b010;
        }
        if self.z >= other.z {
            result = result | 0b001;
        }
        result
    }

    pub fn project_x(&self, other: &Self) -> Self {
        Point3D {
            x: other.x,
            y: self.y,
            z: self.z,
            res: self.res,
        }
    }

    pub fn project_y(&self, other: &Self) -> Self {
        Point3D {
            x: self.x,
            y: other.y,
            z: self.z,
            res: self.res,
        }
    }

    pub fn project_z(&self, other: &Self) -> Self {
        Point3D {
            x: self.x,
            y: self.y,
            z: other.z,
            res: self.res,
        }
    }
}

