/*
 * File: lib.rs
 * Created Date: Saturday August 20th 2022
 * Author: Adrian J. Montero Calvo
 * -----
 * Last Modified:
 * Modified By:
 * -----
 * 2022 Adrián J. Montero Calvo
 * License MIT
 */

mod point3d;
mod ocbb;
mod octree;
mod node;

pub use self::point3d::Point3D;
pub use self::ocbb::OCBoundingBox;
pub use self::octree::{Octree, Octant};
pub use self::node::Node;


#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{Octree, OCBoundingBox, Point3D};
    use rand::Rng;

    #[test]
    fn it_works() {
        // Actually not a test
        let mut _rng = rand::thread_rng();
        let mut now = Instant::now();

        let c0 = Point3D::new(735500.00_f64, 4448000.00_f64, 396.00_f64, 2);
        let c7 = Point3D::new(736499.99_f64, 4448999.99_f64, 398.99_f64, 2);
        let bb0 = OCBoundingBox::new(c0, c7);

        let root = Octree::new(bb0);
        let size = 4000000_usize;
        for _ in 0..size{
            let new_x = rand::thread_rng().gen_range(c0.x..=c7.x);
            let new_y = rand::thread_rng().gen_range(c0.y..=c7.y);
            let new_z = rand::thread_rng().gen_range(c0.z..=c7.z);
            let p3d = Point3D::new(new_x, new_y, new_z, 2);
            // println!("{:?}", p3d);
            root.borrow_mut().insert_point(p3d);
        }
        let filled = format!("****** Filled in  {} seconds ******\n", now.elapsed().as_secs_f32());

        now = Instant::now();
        root.borrow_mut().print();
        let printed = format!("****** Printed in {} seconds ******\n", now.elapsed().as_secs_f32());
        println!("Octree:\n\t{}\t{}", filled, printed);
        assert!(true);
    }

}