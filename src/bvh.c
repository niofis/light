#include <stdlib.h>
#include "list.h"
#include "bvh.h"

typedef enum {X_AXIS, Y_AXIS, Z_AXIS} axis_t;

//https://tavianator.com/fast-branchless-raybounding-box-intersections/
/*
   bool intersection(box b, ray r) {
    double tx1 = (b.min.x - r.x0.x)*r.n_inv.x;
    double tx2 = (b.max.x - r.x0.x)*r.n_inv.x;
 
    double tmin = min(tx1, tx2);
    double tmax = max(tx1, tx2);
 
    double ty1 = (b.min.y - r.x0.y)*r.n_inv.y;
    double ty2 = (b.max.y - r.x0.y)*r.n_inv.y;
 
    tmin = max(tmin, min(ty1, ty2));
    tmax = min(tmax, max(ty1, ty2));
 
    return tmax >= tmin;
}
*/

bvhnode_t *
bvh_build(bvhnode_t *leaves, size_t start, size_t end)
{
  if(start >= end)
    return &leaves[start];

  bvhnode_t *bnode = (bvhnode_t*)malloc(sizeof(bvhnode_t));
  bnode->triangle = NULL;

  //aabb_fit_triangle(&(bnode->bounding_box), leaves[start].triangle);
  aabb_combine(&bnode->bounding_box, &leaves[start].bounding_box, &leaves[end].bounding_box);

  
  for(size_t idx = start + 1; idx <= end; ++idx) {
    bvhnode_t node = leaves[idx];
    aabb_combine(&bnode->bounding_box, &bnode->bounding_box, &node.bounding_box);
  }

  //TODO:
  //find the biggest axis and sort the leaves using that
  float x_length = bnode->bounding_box.max.x - bnode->bounding_box.min.x;
  float y_length = bnode->bounding_box.max.y - bnode->bounding_box.min.y;
  float z_length = bnode->bounding_box.max.z - bnode->bounding_box.min.z;
  
  axis_t axis = x_length < y_length?
    (x_length < z_length?X_AXIS:Z_AXIS):
      (y_length < z_length?Y_AXIS:Z_AXIS);

  //create the left and right branches

  size_t half = (start + end) >> 1;

  bnode->left = bvh_build(leaves, start, half);
  bnode->right = bvh_build(leaves, half + 1, end);

  return bnode;
}

bvh_t*
bvh_new(const list_t *triangles)
{
  bvhnode_t *leaves = (bvhnode_t*)malloc(sizeof(bvhnode_t) * triangles->length);
  size_t idx = 0;
  node_t *node = list_head(triangles);
  
  while(node) {
    bvhnode_t *leave = &leaves[idx];
    aabb_fit_triangle(&(leave->bounding_box), node->item);
    leaves->triangle = node->item;
    leaves->left = NULL;
    leaves->right = NULL;

    ++idx;
    node = list_next(node);
  }

  bvh_t *bvh = (bvh_t*)malloc(sizeof(bvh_t));
  bvh->root = bvh_build(leaves, 0, triangles->length - 1);

  return bvh;
}


