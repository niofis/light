use std::sync::Arc;

use crate::float::Float;
use crate::ilios::bounding_box::BoundingBox;
use crate::ilios::geometry::{Axis, PackedTriangles, Point, Triangle, Vector};
use crate::ilios::ray::Ray;
use crate::ilios::simd;
use crate::ilios::trace::Trace;

#[derive(Clone, Debug)]
pub struct BoundingVolumeHierarchy {
    root: BvhElement,
}

#[derive(Clone, Debug)]
enum BvhElement {
    Empty,
    Node {
        bounding_box: BoundingBox,
        left: Box<BvhElement>,
        right: Box<BvhElement>,
    },
    Leaf {
        primitives: Vec<Arc<Triangle>>,
        packed_primitives: Box<PackedTriangles>,
        bounding_box: BoundingBox,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum BvhBuildMethod {
    Octree,
    Sah,
}

fn rec_trace<'a>(bvh: &'a BvhElement, ray: &Ray, prm_vec: &mut Vec<&'a PackedTriangles>) {
    match bvh {
        BvhElement::Empty => (),
        BvhElement::Node {
            bounding_box,
            left,
            right,
        } => {
            if bounding_box.intersect(ray) {
                rec_trace(left, ray, prm_vec);
                rec_trace(right, ray, prm_vec);
            }
        }
        BvhElement::Leaf {
            primitives: _,
            bounding_box,
            packed_primitives,
        } => {
            if bounding_box.intersect(ray) {
                prm_vec.push(packed_primitives)
            }
        }
    }
}

impl Trace for BoundingVolumeHierarchy {
    fn trace(&self, ray: &Ray) -> Option<Vec<&PackedTriangles>> {
        let mut idx_vec: Vec<&PackedTriangles> = Vec::with_capacity(256);

        rec_trace(&self.root, ray, &mut idx_vec);

        if idx_vec.is_empty() {
            None
        } else {
            Some(idx_vec)
        }
    }
}

fn octree_grouping(items: &[Arc<Triangle>]) -> BvhElement {
    if items.is_empty() {
        return BvhElement::Empty;
    }

    // Termination condition, checks the size of the items for this group
    // and returns a leaf node, which has no left/right children
    if items.len() <= 4 {
        let mut pt = PackedTriangles::default();
        for (i, triangle) in items.iter().enumerate() {
            pt.triangles.push(triangle.clone());
            pt.origin_x = simd::set(pt.origin_x, triangle.origin.0, i);
            pt.origin_y = simd::set(pt.origin_y, triangle.origin.1, i);
            pt.origin_z = simd::set(pt.origin_z, triangle.origin.2, i);
            pt.edge1_x = simd::set(pt.edge1_x, triangle.edge1.0, i);
            pt.edge1_y = simd::set(pt.edge1_y, triangle.edge1.1, i);
            pt.edge1_z = simd::set(pt.edge1_z, triangle.edge1.2, i);
            pt.edge2_x = simd::set(pt.edge2_x, triangle.edge2.0, i);
            pt.edge2_y = simd::set(pt.edge2_y, triangle.edge2.1, i);
            pt.edge2_z = simd::set(pt.edge2_z, triangle.edge2.2, i);
        }
        return BvhElement::Leaf {
            primitives: items.to_vec(),
            packed_primitives: Box::new(pt),
            bounding_box: BoundingBox::default(),
        };
    }

    // Calculate the center for all the items
    let center: Vector = items
        .iter()
        .fold(Point(0.0, 0.0, 0.0), |acc, prm| {
            &prm.bounding_box().centroid + &acc
        })
        .into();
    let center = center / (items.len() as Float);

    // Return the section a particular point is in
    // The section is one of the eight octree regions
    let sector = |c: &Point| if c.0 >= center.0 { 1 } else { 0 } +
        if c.1 >= center.1 { 2 } else { 0 } +
        if c.2 >= center.2 { 4 } else { 0 };

    // Collects all the points for a given sector
    let in_sector = |s| {
        items
            .iter()
            .filter_map(|x| {
                if sector(&x.bounding_box().centroid) == s {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Arc<Triangle>>>()
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
        octree_grouping(
            &items
                .iter()
                .filter_map(|x| {
                    if sectors.contains(&sector(&x.bounding_box().centroid)) {
                        Some(x.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<Arc<Triangle>>>(),
        )
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
    BvhElement::Node {
        bounding_box: BoundingBox::default(),
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn sah_grouping(primitives: &[Arc<Triangle>], total_nodes: &mut usize) -> BvhElement {
    if primitives.is_empty() {
        return BvhElement::Empty;
    }

    *total_nodes += 1;

    if primitives.len() <= 4 {
        let mut pt = PackedTriangles::default();
        for (i, triangle) in primitives.iter().enumerate() {
            pt.triangles.push(triangle.clone());
            pt.origin_x = simd::set(pt.origin_x, triangle.origin.0, i);
            pt.origin_y = simd::set(pt.origin_y, triangle.origin.1, i);
            pt.origin_z = simd::set(pt.origin_z, triangle.origin.2, i);
            pt.edge1_x = simd::set(pt.edge1_x, triangle.edge1.0, i);
            pt.edge1_y = simd::set(pt.edge1_y, triangle.edge1.1, i);
            pt.edge1_z = simd::set(pt.edge1_z, triangle.edge1.2, i);
            pt.edge2_x = simd::set(pt.edge2_x, triangle.edge2.0, i);
            pt.edge2_y = simd::set(pt.edge2_y, triangle.edge2.1, i);
            pt.edge2_z = simd::set(pt.edge2_z, triangle.edge2.2, i);
        }
        return BvhElement::Leaf {
            primitives: primitives.to_vec(),
            packed_primitives: Box::new(pt),
            bounding_box: primitives.iter().fold(BoundingBox::default(), |bb, prm| {
                bb.combine(&prm.bounding_box())
            }),
        };
    }

    let mut min_point = primitives[0].bounding_box().centroid;
    let mut max_point = min_point.clone();

    for primitive in primitives {
        min_point = min_point.min(&primitive.bounding_box().centroid);
        max_point = max_point.max(&primitive.bounding_box().centroid);
    }

    let extents = &max_point - &min_point;

    let selected_axis = if extents.get_component(Axis::X) > extents.get_component(Axis::Y)
        && extents.get_component(Axis::X) > extents.get_component(Axis::Z)
    {
        Axis::X
    } else if extents.get_component(Axis::Y) > extents.get_component(Axis::Z) {
        Axis::Y
    } else {
        Axis::Z
    };

    let extent = extents.get_component(selected_axis);

    // Put all primitives in the different buckets
    const BUCKETS_COUNT: usize = 12;
    let mut buckets: [Vec<Arc<Triangle>>; BUCKETS_COUNT] = Default::default();
    let bucket_size = extent / ((BUCKETS_COUNT - 1) as Float);

    for primitive in primitives {
        let bucket_id = ((primitive
            .bounding_box()
            .centroid
            .get_component(selected_axis)
            - min_point.get_component(selected_axis))
            / bucket_size)
            .floor() as usize;
        buckets[bucket_id].push(primitive.clone());
    }

    // Calculate each bucket bounding box
    let buckets_bb = buckets
        .iter()
        .map(|primitives| {
            primitives.iter().fold(BoundingBox::default(), |bb, prm| {
                bb.combine(&prm.bounding_box())
            })
        })
        .collect::<Vec<BoundingBox>>();

    let partitions_costs = (1..BUCKETS_COUNT)
        .map(|pid| {
            let left_cost = buckets_bb[0..pid]
                .iter()
                .fold(BoundingBox::default(), |a, b| a.combine(b))
                .surface_area()
                * (buckets[0..pid]
                    .iter()
                    .fold(0, |acc, primitives| acc + primitives.len()) as Float);
            let right_cost = buckets_bb[pid..]
                .iter()
                .fold(BoundingBox::default(), |a, b| a.combine(b))
                .surface_area()
                * (buckets[pid..]
                    .iter()
                    .fold(0, |acc, primitives| acc + primitives.len()) as Float);
            left_cost + right_cost
        })
        .collect::<Vec<Float>>();

    let mut split_bucket = 0;
    let mut min_cost = Float::MAX;
    for idx in 0..(BUCKETS_COUNT - 1) {
        let cost = partitions_costs[idx];
        if cost < min_cost {
            min_cost = cost;
            split_bucket = idx + 1;
        }
    }

    let left_group: Vec<Arc<Triangle>> =
        buckets[0..split_bucket]
            .into_iter()
            .fold(Vec::new(), |mut acc, el| {
                acc.append(&mut el.clone());
                acc
            });

    let right_group: Vec<Arc<Triangle>> =
        buckets[split_bucket..]
            .into_iter()
            .fold(Vec::new(), |mut acc, el| {
                acc.append(&mut el.clone());
                acc
            });

    BvhElement::Node {
        bounding_box: primitives.iter().fold(BoundingBox::default(), |bb, prm| {
            bb.combine(&prm.bounding_box())
        }),
        left: Box::new(sah_grouping(&left_group, total_nodes)),
        right: Box::new(sah_grouping(&right_group, total_nodes)),
    }
}

// Will simply recourse the BVH and update the building boxes accordingly
fn rebuild(prms: &[Arc<Triangle>], root: BvhElement, total_nodes: &mut usize) -> BvhElement {
    match root {
        BvhElement::Empty => BvhElement::Empty,
        BvhElement::Node { left, right, .. } => {
            // let t = Arc::get_mut(&mut total_nodes).unwrap();
            *total_nodes += 1;
            let left = rebuild(prms, *left, total_nodes);
            let right = rebuild(prms, *right, total_nodes);
            let mut bounding_box = BoundingBox::default();
            match &left {
                BvhElement::Empty => (),
                BvhElement::Node {
                    bounding_box: bb, ..
                } => bounding_box = bounding_box.combine(bb),
                BvhElement::Leaf {
                    bounding_box: bb, ..
                } => bounding_box = bounding_box.combine(bb),
            }

            match &right {
                BvhElement::Empty => (),
                BvhElement::Node {
                    bounding_box: bb, ..
                } => bounding_box = bounding_box.combine(bb),
                BvhElement::Leaf {
                    bounding_box: bb, ..
                } => bounding_box = bounding_box.combine(bb),
            }

            BvhElement::Node {
                left: Box::new(left),
                right: Box::new(right),
                bounding_box,
            }
        }
        BvhElement::Leaf {
            primitives,
            packed_primitives,
            ..
        } => {
            let bounding_box = primitives.iter().fold(BoundingBox::default(), |acc, p| {
                acc.combine(&p.bounding_box())
            });

            BvhElement::Leaf {
                primitives,
                packed_primitives,
                bounding_box,
            }
        }
    }
}

impl BoundingVolumeHierarchy {
    pub fn new(build_method: BvhBuildMethod, primitives: Vec<Triangle>) -> BoundingVolumeHierarchy {
        let len = primitives.len();
        if len == 0 {
            return BoundingVolumeHierarchy {
                // primitives,
                root: BvhElement::Empty,
            };
        }

        let mut total_nodes: usize = 0;
        let total_primitives = primitives.len();

        let bvh = {
            let prms: Vec<Arc<Triangle>> = primitives.into_iter().map(Arc::new).collect();

            match build_method {
                BvhBuildMethod::Octree => {
                    let root = octree_grouping(&prms);
                    rebuild(&prms, root, &mut total_nodes)
                }
                BvhBuildMethod::Sah => sah_grouping(&prms, &mut total_nodes),
            }
        };
        println!(
            "total_triangles: {}\ntotal_nodes: {}",
            total_primitives, total_nodes
        );
        BoundingVolumeHierarchy { root: bvh }
    }
}
