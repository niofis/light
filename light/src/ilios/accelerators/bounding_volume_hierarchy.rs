use crate::ilios::bounding_box::BoundingBox;
use crate::ilios::geometry::{Point, Triangle, Vector};
use crate::ilios::ray::Ray;
use crate::ilios::trace::Trace;

#[derive(Clone, Debug)]
pub enum Bvh {
    Empty,
    Node {
        primitives: Option<Vec<usize>>,
        bounding_box: BoundingBox,
        left: Box<Bvh>,
        right: Box<Bvh>,
    },
    Leaf {
        primitives: Option<Vec<usize>>,
        bounding_box: BoundingBox,
    },
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

    let first = &items[0];

    // Termination condition, checks the size of the items for this group
    // and returns a leaf node, which has no left/right children
    if items.len() <= 2 || items.iter().all(|pt| pt.0 == first.0) {
        return Bvh::Node {
            primitives: Some(items.iter().map(|x| x.1).collect::<Vec<usize>>()),
            bounding_box: BoundingBox::default(),
            left: Box::new(Bvh::Empty),
            right: Box::new(Bvh::Empty),
        };
    }

    // Calculate the center for all the items
    let center: Vector = items
        .iter()
        .fold(Point(0.0, 0.0, 0.0), |acc, (pt, _)| pt + &acc)
        .into();
    let center = center / (items.len() as f32);

    // Return the section a particular point is in
    // The section is one of the eight octree regions
    let sector = |c: &Point| if c.0 >= center.0 { 1 } else { 0 } +
        if c.1 >= center.1 { 2 } else { 0 } +
        if c.2 >= center.2 { 4 } else { 0 };

    // Collects all the points for a given sector
    let in_sector = |s| {
        items
            .iter()
            .filter_map(|x| if sector(&x.0) == s { Some(*x) } else { None })
            .collect::<Vec<(Point, usize)>>()
    };

    // All the sectors with their corresponding points
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

    // Gets the number of points on each sector
    let lens = sectors.iter().map(|s| s.len() as i64).collect::<Vec<i64>>();

    // Calculates the difference between opposite sectors on a given axis
    let xdiff =
        ((lens[0] + lens[2] + lens[4] + lens[6]) - (lens[1] + lens[3] + lens[5] + lens[7])).abs();
    let ydiff =
        ((lens[0] + lens[1] + lens[4] + lens[5]) - (lens[2] + lens[3] + lens[6] + lens[7])).abs();
    let zdiff =
        ((lens[0] + lens[1] + lens[2] + lens[3]) - (lens[4] + lens[5] + lens[6] + lens[7])).abs();

    let box_for_sectors = |sectors: [usize; 4]| {
        Box::new(octree_grouping(
            &items
                .iter()
                .filter_map(|x| {
                    if sectors.contains(&sector(&x.0)) {
                        Some(*x)
                    } else {
                        None
                    }
                })
                .collect::<Vec<(Point, usize)>>(),
        ))
    };

    // Determinies which axis contains the smallest difference of points
    // This indicates what would be the best splitting position
    // Returns two boxes for this BVH node
    let (left, right) = if xdiff <= ydiff && xdiff <= zdiff {
        (box_for_sectors([0, 2, 4, 6]), box_for_sectors([1, 3, 5, 7]))
    } else if ydiff <= xdiff && ydiff <= zdiff {
        (box_for_sectors([0, 1, 4, 5]), box_for_sectors([2, 3, 6, 7]))
    } else {
        (box_for_sectors([0, 1, 2, 3]), box_for_sectors([4, 5, 6, 7]))
    };

    // The new BVH node with no primitives yet, but a split of space
    // The bounding box has no meaning right now, this is just a grouping of indexes
    Bvh::Node {
        primitives: None,
        bounding_box: BoundingBox::default(),
        left,
        right,
    }
}

fn rebuild(prms: &[Triangle], root: Bvh) -> Bvh {
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
    pub fn new(primitives: &[Triangle]) -> Bvh {
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
