/*
 * File: node.rs
 * Created Date: Thursday August 25th 2022
 * Author: Adrian J. Montero Calvo
 * -----
 * Last Modified:
 * Modified By:
 * -----
 * 2022 Adrian J. Montero Calvo
 * License MIT
 */


use crate::{Point3D, Octant, OCBoundingBox};

pub struct Node {
    pub bb: OCBoundingBox,
    pub points: Vec<Point3D>,
    pub childs: Vec<Option<Octant>>,
}

impl Node {
    pub fn new(bb: OCBoundingBox) -> Node {
        Node {
            bb,
            points: vec![],
            childs: vec![None;8],
        }
    }
}