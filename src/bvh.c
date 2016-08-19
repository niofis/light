#include <stdlib.h>
#include "list.h"
#include "bvh.h"

typedef enum {X_AXIS, Y_AXIS, Z_AXIS} axis_t;

bvh_t *
bvh_build(bvhnode_t *leaves, size_t start, size_t end)
{
  if(start == end)
    return leaves[start];

  bvhnode_t *bnode = (bvhnode_t*)malloc(sizeof(bvhnode_t));

  aabb_fit_triangle(bnode, &leaves[start]->triangle);

  for(size_t idx = start + 1; idx <= end; ++idx) {
    bvhnode_t node = leaves[idx];
    aabb_combine(&bnode->bounding_box, &bnode->bounding_box, &node->bounding_box);
  }

  //TODO:
  //find the biggest axis and sort the leaves using that
  //create the left and right branches

  return NULL;
}

bvh_t*
bvh_new(const list_t *triangles)
{
  bvhnode_t *leaves = (bvhnode_t*)malloc(sizeof(bvhnode_t) * triangles->length);
  size_t idx = 0;
  node_t *node = list_head(triangles);
  
  while(node) {
    aabb_fit_triangle(&leaves[idx]->bounding_box, node->item);
    ++idx;
    node = list_next(triangles);
  }

  return bvh_build(leaves, 0, triangles->length - 1);
}
