use crate::light::bounding_box::BoundingBox;
use crate::light::point::Point;
use crate::light::primitive::Primitive;
use crate::light::ray::Ray;
use crate::light::trace::Trace;
use crate::light::vector::Vector;

#[derive(Debug)]
pub enum Bvh {
    Empty,
    Node {
        primitives: Option<Vec<usize>>,
        bounding_box: BoundingBox,
        left: Box<Bvh>,
        right: Box<Bvh>,
    },
}

#[derive(Debug, Default)]
pub struct BVHStats {
    pub height: usize,
    pub nodes: usize,
    pub leaves: usize,
}

fn rec_trace(bvh: &Bvh, ray: &Ray, prm_vec: &mut Vec<usize>) {
    if let Bvh::Node {
        primitives,
        bounding_box,
        left,
        right,
    } = bvh
    {
        if bounding_box.intersect(ray) {
            if let Some(prms) = primitives {
                for p in prms {
                    prm_vec.push(*p);
                }
            }
            rec_trace(left, ray, prm_vec);
            rec_trace(right, ray, prm_vec);
        }
    };
}

fn in_order_walk(node: &Bvh, mut stats: BVHStats) -> BVHStats {
    match node {
        Bvh::Node {
            left,
            right,
            primitives: _,
            bounding_box: _,
        } => {
            stats.nodes += 1;
            let stats = in_order_walk(left, stats);
            in_order_walk(right, stats)
        }
        _ => stats,
    }
}

impl Bvh {
    pub fn stats(&self) -> BVHStats {
        let stats = BVHStats::default();
        in_order_walk(self, stats)
    }
}

impl Trace for Bvh {
    fn trace(&self, ray: &Ray) -> Option<Vec<usize>> {
        let mut idx_vec: Vec<usize> = Vec::with_capacity(256);

        rec_trace(self, ray, &mut idx_vec);

        if idx_vec.is_empty() {
            None
        } else {
            Some(idx_vec)
        }
    }
}

fn octree_grouping(items: &[(Point, usize)]) -> Bvh {
    if items.is_empty() {
        return Bvh::Empty;
    }

    if items.len() <= 4 {
        return Bvh::Node {
            primitives: Some(items.iter().map(|x| x.1).collect::<Vec<usize>>()),
            bounding_box: BoundingBox::default(),
            left: Box::new(Bvh::Empty),
            right: Box::new(Bvh::Empty),
        };
    }

    let mut minx = std::f32::MAX;
    let mut miny = std::f32::MAX;
    let mut minz = std::f32::MAX;
    let mut maxx = std::f32::MIN;
    let mut maxy = std::f32::MIN;
    let mut maxz = std::f32::MIN;

    for item in items {
        let (Point(x, y, z), _) = item;
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

    let sector = |c: &Point| if c.0 >= center.0 { 1 } else { 0 } +
        if c.1 >= center.1 { 2 } else { 0 } +
        if c.2 >= center.2 { 4 } else { 0 };

    let in_sector = |s| {
        items
            .iter()
            .filter_map(|x| if sector(&x.0) == s { Some(*x) } else { None })
            .collect::<Vec<(Point, usize)>>()
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

    let lens = sectors.iter().map(|s| s.len() as i64).collect::<Vec<i64>>();
    let xdiff =
        ((lens[0] + lens[2] + lens[4] + lens[6]) - (lens[1] + lens[3] + lens[5] + lens[7])).abs();
    let ydiff =
        ((lens[0] + lens[1] + lens[4] + lens[5]) - (lens[2] + lens[3] + lens[6] + lens[7])).abs();
    let zdiff =
        ((lens[0] + lens[1] + lens[2] + lens[3]) - (lens[4] + lens[5] + lens[6] + lens[7])).abs();

    if xdiff <= ydiff && xdiff <= zdiff {
        return Bvh::Node {
            primitives: None,
            bounding_box: BoundingBox::default(),
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
                    .collect::<Vec<(Point, usize)>>(),
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
                    .collect::<Vec<(Point, usize)>>(),
            )),
        };
    } else if ydiff <= xdiff && ydiff <= zdiff {
        return Bvh::Node {
            primitives: None,
            bounding_box: BoundingBox::default(),
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
                    .collect::<Vec<(Point, usize)>>(),
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
                    .collect::<Vec<(Point, usize)>>(),
            )),
        };
    } else {
        return Bvh::Node {
            primitives: None,
            bounding_box: BoundingBox::default(),
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
                    .collect::<Vec<(Point, usize)>>(),
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
                    .collect::<Vec<(Point, usize)>>(),
            )),
        };
    }
}

fn rebuild(prms: &[Primitive], root: Bvh) -> Bvh {
    match root {
        Bvh::Empty => Bvh::Empty,
        Bvh::Node {
            primitives,
            left,
            right,
            ..
        } => {
            let left = rebuild(prms, *left);
            let right = rebuild(prms, *right);
            let mut bounding_box = BoundingBox::default();

            if let Some(indexes) = &primitives {
                bounding_box = indexes.iter().fold(BoundingBox::default(), |acc, p| {
                    acc.combine(&prms[*p].bounding_box())
                });
            }
            if let Bvh::Node {
                bounding_box: lbb, ..
            } = &left
            {
                bounding_box = bounding_box.combine(lbb);
            }

            if let Bvh::Node {
                bounding_box: rbb, ..
            } = &right
            {
                bounding_box = bounding_box.combine(rbb);
            }

            Bvh::Node {
                primitives,
                left: Box::new(left),
                right: Box::new(right),
                bounding_box,
            }
        }
    }
}

impl Bvh {
    pub fn new(primitives: &[Primitive]) -> Bvh {
        let len = primitives.len();
        if len == 0 {
            return Bvh::Empty;
        }

        let indexes = 0..len;
        let centroid = primitives.iter().map(|x| x.bounding_box().centroid);
        let items: Vec<(Point, usize)> = centroid.zip(indexes).collect();
        let root = octree_grouping(&items);
        rebuild(primitives, root)
    }
}
