#pragma once
#include "includes.h"

typedef struct _bvhnode_t {
  aabb_t bounding_box;
  triangle_t *triangle;
  struct _bvhnode_t *left;
  struct _bvhnode_t *right;
} bvhnode_t;

typedef struct {
  bvhnode_t *root;  
  bvhnode_t *leaves;
} bvh_t;


bvh_t*
  bvh_new(const list_t *triangles);

void
  bvh_destroy(bvh_t **bvh);
