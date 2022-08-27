/*
 * File: octree.rs
 * Created Date: Thursday August 25th 2022
 * Author: Adrian J. Montero Calvo
 * -----
 * Last Modified:
 * Modified By:
 * -----
 * 2022 Adrian J. Montero Calvo
 * License MIT
 */

use std::rc::Rc;
use std::{cell::RefCell, io::Write};

use crate::{Node, OCBoundingBox, Point3D};

pub type Octant = Rc<RefCell<Octree>>;

pub struct Octree {
    pub root: Node,
}

impl Octree {
    pub fn new(bb: OCBoundingBox) -> Octant {
        Rc::new(RefCell::new(Octree {
            root: Node::new(bb),
        }))
    }
}
impl Octree {
    pub fn insert_point(&mut self, point: Point3D) {
        let current = &mut self.root;
        match &current.childs[0] {
            // Child Octants have been created. No None.
            Some(oct) => {
                let p_root = oct.borrow().root.bb.c7.to_owned();
                let idx = point.rel_position(&p_root);
                // I already know its existence, so unwrap
                current.childs[idx]
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .insert_point(point);
            }
            // Child Octans have not been created. May exist up to 8 point3d in
            // points vector.
            None => {
                match &current.points.len() {
                    8 => {
                        let bbs = &current.bb.bud();
                        for i in 0..8_usize {
                            let _ = current.childs[i].insert(Octree::new(bbs[i].clone()));
                        }
                        let p_childs_root = &current.childs[0]
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .root
                            .bb
                            .c7
                            .to_owned();
                        for point3d in &current.points {
                            // I've just created, so exists and unwrap
                            let idx = point3d.rel_position(p_childs_root);
                            current.childs[idx]
                                .as_ref()
                                .unwrap()
                                .borrow_mut()
                                .insert_point(*point3d);
                        }
                        current.points.clear();
                        // I've created its container, so let's go with unwrap
                        let idx = point.rel_position(p_childs_root);
                        current.childs[idx]
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .insert_point(point);
                    }
                    _ => {
                        // Here is where all points fall
                        current.points.push(point);
                    }
                }
            }
        }
    }

    pub fn print(&mut self) {
        let current = &mut self.root;

        for i in &current.childs {
            if !i.is_none() {
                i.to_owned().unwrap().borrow_mut().print();
            }
        }
        println!("{:?} -> {:?}: ", &current.bb.c0, &current.bb.c7);
        std::io::stdout().flush().unwrap();
        for point in &current.points {
            println!("\t{:?} ", point);
            std::io::stdout().flush().unwrap();
        }
    }
}
