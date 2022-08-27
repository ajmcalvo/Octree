/*
 * File: ocbb.rs
 * Created Date: Thursday August 25th 2022
 * Author: Adrian J. Montero Calvo
 * -----
 * Last Modified:
 * Modified By:
 * -----
 * 2022 Adrian J. Montero Calvo
 * License MIT
 */


use crate::Point3D;

#[derive(PartialEq, PartialOrd, Clone, Debug, Default)]
pub struct OCBoundingBox {
    pub c0: Point3D, // West South Bottom
    pub c7: Point3D, // East North Up
}

impl OCBoundingBox {
    pub fn new(c0: Point3D, c7: Point3D) -> Self {
        Self { c0, c7 }
    }

    pub fn bud(&self) -> Vec<Self> {
        let c0_7 = self.c0.middle(&self.c7);
        let c0_5 = c0_7.project_y(&self.c0);
        let c0_4 = c0_5.project_z(&self.c0);
        let c0_1 = self.c0.project_z(&c0_5);
        let c0_3 = c0_1.project_y(&c0_7);
        let c0_2 = c0_3.project_z(&c0_4);
        let c0_6 = c0_4.project_y(&c0_2);

        let c7_7 = self.c7.clone();
        let c7_4 = c0_7.project_x(&self.c7);
        let c7_6 = c7_4.project_y(&c7_7);
        let c7_5 = c7_4.project_z(&c7_7);
        let c7_1 = c7_5.project_x(&c0_7);
        let c7_3 = c7_1.project_y(&c7_7);
        let c7_2 = c7_6.project_x(&c7_3);

        vec![
            Self {
                c0: self.c0.clone(),
                c7: c0_7,
            },
            Self { c0: c0_1, c7: c7_1 },
            Self { c0: c0_2, c7: c7_2 },
            Self { c0: c0_3, c7: c7_3 },
            Self { c0: c0_4, c7: c7_4 },
            Self { c0: c0_5, c7: c7_5 },
            Self { c0: c0_6, c7: c7_6 },
            Self { c0: c0_7, c7: c7_7 },
        ]
    }

    
}

#[cfg(test)]
mod tests {
    use crate::{OCBoundingBox, Point3D};

    /*  (z)
        2          20 (y)
        1     12
        0 4
         10   20   30 (x)  
        
            ------------- (c7-c7_7)
           /  3  /  7   /|
          /_____/______/ |
         /     /      /| |
        /-----/------/ |/|
        |     |      | |6|
        | 1   |   5  |/| /
        |-----|------| |/
        | 0   |   4  | /
c0-c0_7 |_____|______|/
*/

    fn get_bbs1() -> Vec<OCBoundingBox>{
        let c00 = Point3D::new(10_f64, 4_f64, 0_f64, 1);
        let c07 = Point3D::new(20_f64, 12_f64, 1_f64, 1);
        let c10 = Point3D::new(10_f64, 4_f64, 1_f64, 1);
        let c17 = Point3D::new(20_f64, 12_f64, 2_f64, 1);
        let c20 = Point3D::new(10_f64, 12_f64, 0_f64, 1);
        let c27 = Point3D::new(20_f64, 20_f64, 1_f64, 1);
        let c30 = Point3D::new(10_f64, 12_f64, 1_f64, 1);
        let c37 = Point3D::new(20_f64, 20_f64, 2_f64, 1);
        let c40 = Point3D::new(20_f64, 4_f64, 0_f64, 1);
        let c47 = Point3D::new(30_f64, 12_f64, 1_f64, 1);
        let c50 = Point3D::new(20_f64, 4_f64, 1_f64, 1);
        let c57 = Point3D::new(30_f64, 12_f64, 2_f64, 1);
        let c60 = Point3D::new(20_f64, 12_f64, 0_f64, 1);
        let c67 = Point3D::new(30_f64, 20_f64, 1_f64, 1);
        let c70 = Point3D::new(20_f64, 12_f64, 1_f64, 1);
        let c77 = Point3D::new(30_f64, 20_f64, 2_f64, 1);
        let ocb0 = OCBoundingBox::new(c00, c07);
        let ocb1 = OCBoundingBox::new(c10, c17);
        let ocb2 = OCBoundingBox::new(c20, c27);
        let ocb3 = OCBoundingBox::new(c30, c37);
        let ocb4 = OCBoundingBox::new(c40, c47);
        let ocb5 = OCBoundingBox::new(c50, c57);
        let ocb6 = OCBoundingBox::new(c60, c67);
        let ocb7 = OCBoundingBox::new(c70, c77);
        vec![ocb0, ocb1, ocb2, ocb3, ocb4, ocb5, ocb6, ocb7]
    }

    #[test]
    fn it_works() {
        let c0 = Point3D::new(10_f64, 4_f64, 0_f64, 1);
        let c7 = Point3D::new(30_f64, 20_f64, 2_f64, 1);
        let ocb = OCBoundingBox::new(c0, c7);
        let children = ocb.bud();
        let bbs1 = get_bbs1();

        assert!(children == bbs1);
    }
}