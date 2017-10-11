#pragma once
#include "includes.h"

typedef struct _bvhnode_t {
  aabb_t bb;
  primitive_t *primitive;
  struct _bvhnode_t *left;
  struct _bvhnode_t *right;
} bvhnode_t;

typedef struct {
  float *min_x;
  float *min_y;
  float *min_z;
  float *max_x;
  float *max_y;
  float *max_z;
  primitive_t **primitives;
  size_t length;
} bvh_heap_t;

typedef struct {
  bvhnode_t *root;  
  bvhnode_t *leaves;
  size_t length;
} bvh_t;

bvh_t*
  bvh_new(const list_t *triangles);

void
  bvh_destroy(bvh_t **bvh);
