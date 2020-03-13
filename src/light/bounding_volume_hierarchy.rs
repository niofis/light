use crate::light::bounding_box::*;
use crate::light::primitive::*;
use crate::light::ray::*;
use crate::light::trace::*;
use crate::light::vector::*;

#[derive(Debug)]
pub enum BVHNode {
    Empty,
    Node {
        primitives: Option<Vec<usize>>,
        bounding_box: BoundingBox,
        left: Box<BVHNode>,
        right: Box<BVHNode>,
    },
}

#[derive(Debug)]
pub struct BVH {
    primitives: Vec<Primitive>,
    root: BVHNode,
}

fn rec_trace(bvh: &BVHNode, ray: &Ray, prm_vec: &mut Vec<usize>) {
    match bvh {
        BVHNode::Node {
            primitives,
            bounding_box,
            left,
            right,
        } => {
            if bounding_box.intersect(ray) {
                if let Some(prms) = primitives {
                    //prm_vec.append(&mut prms.iter().map(|p| *p).collect::<Vec<usize>>());
                    for p in prms {
                        prm_vec.push(*p);
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
        let BVH { primitives, root } = self;
        let mut idx_vec: Vec<usize> = Vec::new();

        rec_trace(&root, &ray, &mut idx_vec);

        if idx_vec.is_empty() {
            None
        } else {
            let prm_vec = idx_vec.iter().map(|i| &primitives[*i]).collect();
            Some(prm_vec)
        }
    }
}

/*
fn create_hierarchy(primitives: &Vec<Primitive>, mut indexes: Vec<usize>) -> BVHNode {
    let len = indexes.len();

    let bb = indexes.iter().fold(BoundingBox::empty(), |acc, p| {
        acc.combine(&primitives[*p].bounding_box())
    });

    if len <= 1 {
        return BVHNode::Node {
            primitives: Some(indexes),
            bounding_box: bb,
            left: Box::new(BVHNode::Empty),
            right: Box::new(BVHNode::Empty),
        };
    }

    let mid = len / 2;

    let right = indexes.split_off(mid);

    return BVHNode::Node {
        primitives: None,
        bounding_box: bb,
        left: Box::new(create_hierarchy(&primitives, indexes)),
        right: Box::new(create_hierarchy(&primitives, right)),
    };
}
*/

fn octree_grouping(items: &Vec<(Vector, usize)>) -> BVHNode {
    if items.len() == 0 {
        return BVHNode::Empty;
    }

    if items.len() <= 1 {
        return BVHNode::Node {
            primitives: Some(items.iter().map(|x| x.1).collect::<Vec<usize>>()),
            bounding_box: BoundingBox::empty(),
            left: Box::new(BVHNode::Empty),
            right: Box::new(BVHNode::Empty),
        };
    }

    let mut minx = std::f32::MAX;
    let mut miny = std::f32::MAX;
    let mut minz = std::f32::MAX;
    let mut maxx = std::f32::MIN;
    let mut maxy = std::f32::MIN;
    let mut maxz = std::f32::MIN;

    for item in items {
        let (Vector(x, y, z), _) = item;
        minx = if *x < minx { *x } else { minx };
        miny = if *y < miny { *y } else { miny };
        minz = if *z < minz { *z } else { minz };

        maxx = if *x > maxx { *x } else { maxx };
        maxy = if *y > maxy { *y } else { maxy };
        maxz = if *z > maxz { *z } else { maxz };
    }

    let center = Vector(
        (minx + maxx) / 2.0,
        (miny + maxy) / 2.0,
        (minz + maxz) / 2.0,
    );

    let sector = |c: &Vector| if c.0 >= center.0 { 1 } else { 0 } +
        if c.1 >= center.1 { 2 } else { 0 } +
        if c.2 >= center.2 { 4 } else { 0 };

    let in_sector = |s| {
        items
            .iter()
            .filter_map(|x| if sector(&x.0) == s { Some(*x) } else { None })
            .collect::<Vec<(Vector, usize)>>()
    };

    let sectors = [
        in_sector(0),
        in_sector(1),
        in_sector(2),
        in_sector(3),
        in_sector(4),
        in_sector(5),
        in_sector(6),
        in_sector(7),
    ];

    let lens = sectors
        .iter()
        .map(|s| s.len() as i64)
        .collect::<Vec<i64>>();
    let xdiff =
        ((lens[0] + lens[2] + lens[4] + lens[6]) - (lens[1] + lens[3] + lens[5] + lens[7])).abs();
    let ydiff =
        ((lens[0] + lens[1] + lens[4] + lens[5]) - (lens[2] + lens[3] + lens[6] + lens[7])).abs();
    let zdiff =
        ((lens[0] + lens[1] + lens[2] + lens[3]) - (lens[4] + lens[5] + lens[6] + lens[7])).abs();

    if xdiff <= ydiff && xdiff <= zdiff {
        return BVHNode::Node {
            primitives: None,
            bounding_box: BoundingBox::empty(),
            left: Box::new(octree_grouping(
                &items
                    .iter()
                    .filter_map(|x| {
                        if [0, 2, 4, 6].contains(&sector(&x.0)) {
                            Some(*x)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(Vector, usize)>>(),
            )),
            right: Box::new(octree_grouping(
                &items
                    .iter()
                    .filter_map(|x| {
                        if [1, 3, 5, 7].contains(&sector(&x.0)) {
                            Some(*x)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(Vector, usize)>>(),
            )),
        };
    } else if ydiff <= xdiff && ydiff <= zdiff {
        return BVHNode::Node {
            primitives: None,
            bounding_box: BoundingBox::empty(),
            left: Box::new(octree_grouping(
                &items
                    .iter()
                    .filter_map(|x| {
                        if [0, 1, 4, 5].contains(&sector(&x.0)) {
                            Some(*x)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(Vector, usize)>>(),
            )),
            right: Box::new(octree_grouping(
                &items
                    .iter()
                    .filter_map(|x| {
                        if [2, 3, 6, 7].contains(&sector(&x.0)) {
                            Some(*x)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(Vector, usize)>>(),
            )),
        };
    } else {
        return BVHNode::Node {
            primitives: None,
            bounding_box: BoundingBox::empty(),
            left: Box::new(octree_grouping(
                &items
                    .iter()
                    .filter_map(|x| {
                        if [0, 1, 2, 3].contains(&sector(&x.0)) {
                            Some(*x)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(Vector, usize)>>(),
            )),
            right: Box::new(octree_grouping(
                &items
                    .iter()
                    .filter_map(|x| {
                        if [4, 5, 6, 7].contains(&sector(&x.0)) {
                            Some(*x)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(Vector, usize)>>(),
            )),
        };
    }
}

fn rebuild(prms: &Vec<Primitive>, root: BVHNode) -> BVHNode {
    match root {
        BVHNode::Empty => BVHNode::Empty,
        BVHNode::Node {
            primitives,
            left,
            right,
            ..
        } => {
            let left = rebuild(&prms, *left);
            let right = rebuild(&prms, *right);
            let mut bounding_box = BoundingBox::empty();

            if let Some(indexes) = &primitives {
                bounding_box = indexes.iter().fold(BoundingBox::empty(), |acc, p| {
                    acc.combine(&prms[*p].bounding_box())
                });
            }
            if let BVHNode::Node {
                bounding_box: lbb, ..
            } = &left
            {
                bounding_box = bounding_box.combine(&lbb);
            }

            if let BVHNode::Node {
                bounding_box: rbb, ..
            } = &right
            {
                bounding_box = bounding_box.combine(&rbb);
            }

            BVHNode::Node {
                primitives,
                left: Box::new(left),
                right: Box::new(right),
                bounding_box,
            }
        }
    }
}

impl BVH {
    pub fn new(primitives: Vec<Primitive>) -> BVH {
        let len = primitives.len();
        if len == 0 {
            return BVH {
                primitives,
                root: BVHNode::Empty,
            };
        }

        let indexes: Vec<usize> = (0..len).collect();
        let centroid: Vec<Vector> = primitives.iter().map(|x| x.centroid()).collect();
        let mut items: Vec<(Vector, usize)> =
            centroid.into_iter().zip(indexes.into_iter()).collect();
        //let root = create_hierarchy(&primitives, indexes);
        let root = octree_grouping(&mut items);
        let root = rebuild(&primitives, root);

        BVH { primitives, root }
    }

    /*
    pub fn stats(&self) -> (usize, usize, usize) {
        let BVH { root, .. } = self;
        let mut count = 0;
        let mut arity = 0;
        let mut height = 0;
        let mut stack = VecDeque::new();
        stack.push_back(root);

        while !stack.is_empty() {
            height = height.max(stack.len());
            let bvh = stack.pop_back();
            match bvh {
                Some(BVHNode::Node {
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
    */
}
