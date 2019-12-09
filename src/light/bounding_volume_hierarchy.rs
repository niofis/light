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

impl Trace for BVH {
    fn trace<'a>(&self, ray: &Ray) -> Option<Vec<&Primitive>> {
        let mut prm_vec: Vec<&Primitive> = Vec::new();
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
                            prm_vec = prms.iter().fold(prm_vec, |mut acc, p| {
                                acc.push(&p);
                                acc
                            });
                        }
                        stack.push_back(right);
                        stack.push_back(left);
                    }
                }
                _ => {}
            }
        }

        if prm_vec.len() > 0 {
            return Some(prm_vec);
        }
        None
    }
}

impl BVH {
    pub fn new(mut primitives: Vec<Primitive>) -> BVH {
        let len = primitives.len();
        if len == 0 {
            return BVH::Empty;
        }

        let bb = primitives.iter().fold(BoundingBox::empty(), |acc, p| {
            acc.combine(&p.bounding_box())
        });

        if len <= 10 {
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

    pub fn stats(&self) -> (usize, usize) {
        let mut count = 0;
        let mut arity = 0;
        let mut stack = VecDeque::new();
        stack.push_back(self);

        while !stack.is_empty() {
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
        (count, arity)
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
