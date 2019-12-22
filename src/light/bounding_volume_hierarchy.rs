use crate::light::bounding_box::*;
use crate::light::primitive::*;
use crate::light::ray::*;
use crate::light::trace::*;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum BVH {
    Empty,
    Node {
        primitives: Option<Vec<Primitive>>,
        bounding_box: BoundingBox,
        left: Box<BVH>,
        right: Box<BVH>,
    },
}

fn rec_trace<'a>(bvh: &'a BVH, ray: &Ray, prm_vec: &mut Vec<&'a Primitive>) {
    match bvh {
        BVH::Node {
            primitives,
            bounding_box,
            left,
            right,
        } => {
            if bounding_box.intersect(ray) {
                if let Some(prms) = primitives {
                    //let mut coso = prms.iter().map(|p| p).collect::<Vec<&Primitive>>();
                    
                    //prm_vec.append(&mut prms.iter().map(|p| p).collect::<Vec<&Primitive>>());

                    for p in prms {
                        prm_vec.push(p);
                    }
                }
                rec_trace(&left, ray, prm_vec);
                rec_trace(&right, ray, prm_vec);
            }
        }
        _ => {}
    };
}

impl Trace for BVH {
    fn trace(&self, ray: &Ray) -> Option<Vec<&Primitive>> {
        let mut prm_vec: Vec<&Primitive> = Vec::new();

        rec_trace(&self, &ray, &mut prm_vec);

        /*
        let mut stack = VecDeque::new();
        stack.push_back(self);

        while !stack.is_empty() {
            let bvh = stack.pop_back();
            match bvh {
                Some(BVH::Node {
                    primitives,
                    bounding_box,
                    left,
                    right,
                }) => {
                    if bounding_box.intersect(ray) {
                        if let Some(prms) = primitives {
                            for p in prms {
                                prm_vec.push(&p);
                            }
                        }
                        stack.push_back(right);
                        stack.push_back(left);
                    }
                }
                _ => {}
            }
        }*/

        if prm_vec.is_empty() {
            None
        } else {
            Some(prm_vec)
        }
    }
}

impl BVH {
    pub fn new(mut primitives: Vec<Primitive>) -> BVH {
        let len = primitives.len();
        if len == 0 {
            return BVH::Empty;
        }

        //primitives.sort_by(|a, b| a.centroid().0.partial_cmp(&b.centroid().0).unwrap());

        let bb = primitives.iter().fold(BoundingBox::empty(), |acc, p| {
            acc.combine(&p.bounding_box())
        });

        if len <= 1 {
            return BVH::Node {
                primitives: Some(primitives),
                bounding_box: bb,
                left: Box::new(BVH::Empty),
                right: Box::new(BVH::Empty),
            };
        }

        let mid = len / 2;

        let right = primitives.split_off(mid);

        return BVH::Node {
            primitives: None,
            bounding_box: bb,
            left: Box::new(BVH::new(primitives)),
            right: Box::new(BVH::new(right)),
        };
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        let mut count = 0;
        let mut arity = 0;
        let mut height = 0;
        let mut stack = VecDeque::new();
        stack.push_back(self);

        while !stack.is_empty() {
            height = height.max(stack.len());
            let bvh = stack.pop_back();
            match bvh {
                Some(BVH::Node {
                    primitives,
                    left,
                    right,
                    ..
                }) => {
                    arity = arity + 1;
                    if let Some(prms) = primitives {
                        count = count + prms.len();
                    }
                    stack.push_back(right);
                    stack.push_back(left);
                }
                _ => {}
            }
        }
        (count, arity, height)
    }
}

pub struct BVHIterator<'a> {
    stack: VecDeque<Box<&'a BVH>>,
}

impl BVHIterator<'_> {
    pub fn new<'a>(bvh: &'a BVH) -> BVHIterator<'a> {
        let mut stack = VecDeque::new();
        stack.push_back(Box::new(bvh));
        BVHIterator { stack }
    }
}

impl Iterator for BVHIterator<'_> {
    type Item = Vec<Primitive>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
