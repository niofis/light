#pragma once

#include "vector3.h"
#include "triangle.h"

typedef struct {
  v3_t min;
  v3_t max;
  v3_t centroid;
} aabb_t;

aabb_t*
  aabb_from_triangle(triangle_t* triangle);

aabb_t*
  aabb_add(aabb_t *dest, aabb_t *bb1, aabb_t *bb2);

void
  aabb_destroy(aabb_t **bb);
