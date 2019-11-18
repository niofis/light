use crate::light::bounding_box::*;
use crate::light::primitive::*;

#[derive(Debug)]
pub enum BVH<'a> {
    Empty,
    Node {
        primitives: Option<&'a [Primitive]>,
        bounding_box: BoundingBox,
        left: Box<BVH<'a>>,
        right: Box<BVH<'a>>,
    },
}

impl BVH<'_> {
    pub fn new<'a>(primitives: &'a [Primitive]) -> BVH {
        let len = primitives.len();
        if len == 0 {
            return BVH::Empty;
        }

        let bb = primitives.iter().fold(BoundingBox::empty(), |acc, p| {
            acc.combine(&p.bounding_box())
        });
        if len <= 4 {
            return BVH::Node {
                primitives: Some(primitives),
                bounding_box: bb,
                left: Box::new(BVH::Empty),
                right: Box::new(BVH::Empty),
            };
        }

        let mid = len / 2;

        return BVH::Node {
            primitives: None,
            bounding_box: bb,
            left: Box::new(BVH::new(&primitives[0..mid])),
            right: Box::new(BVH::new(&primitives[mid..])),
        };

        //println!("{}", mid);
    }
}
