#pragma once
#include "includes.h"

typedef struct {
  v3_t min;
  v3_t max;
  v3_t centroid;
} aabb_t;

aabb_t*
  aabb_new_from_triangle(triangle_t* triangle);

aabb_t*
  aabb_new_from_sphere(sphere_t* sphere);

aabb_t*
  aabb_new_from_primitive(primitive_t *prm);

void
  aabb_fit_triangle(aabb_t *bb, triangle_t* triangle);

void
  aabb_combine(aabb_t *dest, aabb_t *bb1, aabb_t *bb2);

void
  aabb_update_centroid(aabb_t *bb);

void
  aabb_destroy(aabb_t **bb);

int
  aabb_intersect(aabb_t *bb, ray_t *ray);
