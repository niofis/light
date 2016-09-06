#pragma once

#include "aabb.h"
#include "triangle.h"
#include "list.h"

typedef struct _bvhnode_t {
  aabb_t bounding_box;
  triangle_t *triangle;
  struct _bvhnode_t *left;
  struct _bvhnode_t *right;
} bvhnode_t;

typedef struct {
  bvhnode_t *root;  
} bvh_t;


bvh_t*
  bvh_new(const list_t *triangles);