#include "includes.h"

typedef enum {X_AXIS, Y_AXIS, Z_AXIS} axis_t;

/*
void
bvh_mkh_helper(bvh_heap_t *heap, bvhnode_t *node, size_t idx)
{
  heap->min_x[idx] = node->bb.min.x;
  heap->min_y[idx] = node->bb.min.y;
  heap->min_z[idx] = node->bb.min.z;

  heap->max_x[idx] = node->bb.max.x;
  heap->max_y[idx] = node->bb.max.y;
  heap->max_z[idx] = node->bb.max.z;
  heap->primitives[idx] = node->primitive;

  if(node->left)
    bvh_mkh_helper(heap, node->left, (idx * 2) + 1);
  if(node->right)
    bvh_mkh_helper(heap, node->right, (idx * 2) + 2);
}

void
bvh_make_heap(bvh_t *bvh)
{
  bvh_heap_t *heap = (bvh_heap_t*) malloc(sizeof(bvh_heap_t));
  size_t count = (size_t) pow(2,ceil(log2(bvh->length)));
  count += count - 1;
  heap->min_x = (float*) malloc(sizeof(float) * count);
  heap->min_y = (float*) malloc(sizeof(float) * count);
  heap->min_z = (float*) malloc(sizeof(float) * count);
  heap->max_x = (float*) malloc(sizeof(float) * count);
  heap->max_y = (float*) malloc(sizeof(float) * count);
  heap->max_z = (float*) malloc(sizeof(float) * count);
  heap->primitives = (primitive_t**) malloc(sizeof(primitive_t*) * count);
  heap->length = count;

  bvh->heap = heap;

  bvh_mkh_helper(heap, bvh->root, 0);
}
*/

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
          leaves[b-1].bb.centroid.x > temp.bb.centroid.x) {
        leaves[b] = leaves[b-1];
      } if (axis == Y_AXIS &&
          leaves[b-1].bb.centroid.y > temp.bb.centroid.y) {
        leaves[b] = leaves[b-1];
      } if (axis == Z_AXIS &&
          leaves[b-1].bb.centroid.z > temp.bb.centroid.z) {
        leaves[b] = leaves[b-1];
      } else {
        break;
      }
    }
    leaves[b] = temp;
  }
}

bvhnode_t *
bvh_build(bvh_t *bvh, size_t start, size_t end)
{

  if(start >= end)
    return &(bvh->leaves[start]);

  bvhnode_t *bnode = (bvhnode_t*)malloc(sizeof(bvhnode_t));
  bnode->primitive = NULL;
  bnode->left = NULL;
  bnode->right = NULL;
  v3_copy(&bnode->bb.min, &bvh->leaves[start].bb.min);
  v3_copy(&bnode->bb.max, &bvh->leaves[start].bb.max);

  //aabb_fit_triangle(&(bnode->bb), leaves[start].triangle);
  //aabb_combine(&bnode->bb, &(bvh->leaves[start].bb), &(bvh->leaves[end].bb));

  
  for(size_t idx = start + 1; idx <= end; ++idx) {
    bvhnode_t *node = &bvh->leaves[idx];
    aabb_combine(&bnode->bb, &bnode->bb, &node->bb);
  }

  //find the biggest axis and sort the leaves using that
  float x_length = bnode->bb.max.x - bnode->bb.min.x;
  float y_length = bnode->bb.max.y - bnode->bb.min.y;
  float z_length = bnode->bb.max.z - bnode->bb.min.z;
  
  axis_t axis = x_length < y_length?
    (x_length < z_length?X_AXIS:Z_AXIS):
      (y_length < z_length?Y_AXIS:Z_AXIS);

  //create the left and right branches
  //printf("axis: %u\n", (unsigned int)axis);
  sort_leaves(bvh->leaves, start, end, axis);
  
  size_t half = (start + end) >> 1;

  bnode->left = bvh_build(bvh, start, half);
  bnode->right = bvh_build(bvh, half + 1, end);


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
    aabb_fit_primitive(&(leave->bb), node->item);
    leave->primitive = node->item;
    leave->left = NULL;
    leave->right = NULL;
/*
    printf("min: %f\n", min3(1.0f,2.0f,3.0f));
    printf("min: %f\n", min3(2.0f,1.0f,3.0f));
    printf("min: %f\n", min3(2.0f,3.0f,1.0f));
    printf("max: %f\n", max3(1.0f,2.0f,3.0f));
    printf("max: %f\n", max3(2.0f,1.0f,3.0f));
    printf("max: %f\n", max3(2.0f,3.0f,1.0f));
    */
    //printf("min(%f, %f, %f) max(%f, %f, %f)\n", leave->bb.min.x, leave->bb.min.y, leave->bb.min.z,leave->bb.max.x, leave->bb.max.y, leave->bb.max.z);



    ++idx;
    node = list_next(node);
  }

  bvh_t *bvh = (bvh_t*)malloc(sizeof(bvh_t));
  bvh->leaves = leaves;
  bvh->length = idx;
  bvh->root = bvh_build(bvh, 0, primitives->length - 1);
  //bvh_make_heap(bvh);

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

/*
void
bvh_destroy_heap(bvh_heap_t **heap)
{
  bvh_heap_t *hp = *heap;
  free(hp->min_x);
  free(hp->min_y);
  free(hp->min_z);

  free(hp->max_x);
  free(hp->max_y);
  free(hp->max_z);

  free(hp->primitives);

  free(hp);
  hp = NULL;
}
*/
void
bvh_destroy(bvh_t **bvh)
{
  bvh_t *b = *bvh;
  bvh_destroy_children(&(b->root));
  //bvh_destroy_heap(&(b->heap));
  free(b->leaves);
  free(*bvh);
  *bvh = NULL;
}
