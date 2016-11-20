#include "includes.h"

typedef enum {X_AXIS, Y_AXIS, Z_AXIS} axis_t;

void
bvh_make_heap

void
sort_leaves(bvhnode_t* leaves, size_t start, size_t end, axis_t axis)
{
  size_t a;
  size_t b;
  bvhnode_t temp;
  //insertion sort
  for(a = start + 1; a <= end; a++) {
    temp = leaves[a];
    for(b = a; b > start; b--) {
      if (axis == X_AXIS &&
          leaves[b-1].bounding_box.centroid.x > temp.bounding_box.centroid.x) {
        leaves[b] = leaves[b-1];
      } if (axis == Y_AXIS &&
          leaves[b-1].bounding_box.centroid.y > temp.bounding_box.centroid.y) {
        leaves[b] = leaves[b-1];
      } if (axis == Z_AXIS &&
          leaves[b-1].bounding_box.centroid.z > temp.bounding_box.centroid.z) {
        leaves[b] = leaves[b-1];
      } else {
        break;
      }
    }
    leaves[b] = temp;
  }
}

bvhnode_t *
bvh_build(bvhnode_t *leaves, size_t start, size_t end)
{

  if(start >= end)
    return &leaves[start];

  bvhnode_t *bnode = (bvhnode_t*)malloc(sizeof(bvhnode_t));
  bnode->primitive = NULL;
  bnode->left = NULL;
  bnode->right = NULL;

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
  //printf("axis: %u\n", (unsigned int)axis);
  sort_leaves(leaves, start, end, axis);
  
  size_t half = (start + end) >> 1;

  bnode->left = bvh_build(leaves, start, half);
  bnode->right = bvh_build(leaves, half + 1, end);

  return bnode;
}

bvh_t*
bvh_new(const list_t *primitives)
{
  bvhnode_t *leaves = (bvhnode_t*)malloc(sizeof(bvhnode_t) * primitives->length);
  size_t idx = 0;
  node_t *node = list_head(primitives);
  
  while(node) {
    bvhnode_t *leave = &(leaves[idx]);
    aabb_fit_primitive(&(leave->bounding_box), node->item);
    leave->primitive = node->item;
    leave->left = NULL;
    leave->right = NULL;

    ++idx;
    node = list_next(node);
  }

  bvh_t *bvh = (bvh_t*)malloc(sizeof(bvh_t));
  bvh->root = bvh_build(leaves, 0, primitives->length - 1);
  bvh->leaves = leaves;

  //v3_t min = bvh->root->bounding_box.min;
  //v3_t max = bvh->root->bounding_box.max;
  //printf("min(%f, %f, %f) max(%f,%f,%f)\n",
  //    min.x,min.y,min.z,
  //    max.x,max.y,max.z);
  return bvh;
}

void
bvh_destroy_children(bvhnode_t **node)
{
  bvhnode_t *n = *node;
  //Its a leaf, ignore
  if(!n || (n->left == NULL & n->right == NULL))
    return;
  if(n->left)
    bvh_destroy_children(&(n->left));
  if(n->right)
    bvh_destroy_children(&(n->right));
  free(*node);
  *node = NULL;
}

void
bvh_destroy(bvh_t **bvh)
{
  bvh_t *b = *bvh;
  bvh_destroy_children(&(b->root));
  free(b->leaves);
  free(*bvh);
  *bvh = NULL;
}
