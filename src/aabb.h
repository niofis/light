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

void
  aabb_combine(aabb_t *dest, aabb_t *bb1, aabb_t *bb2);

void
  aabb_update_centroid(aabb_t *bb);

void
  aabb_destroy(aabb_t **bb);
