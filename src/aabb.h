#pragma once
#include "includes.h"

typedef struct {
  v3_t min;
  v3_t max;
  v3_t centroid;
} aabb_t;

#define min(a,b) (a<b?a:b)
#define max(a,b) (a>b?a:b)
#define min3(a,b,c) (a<b?(a<c?a:c):(b<c?b:c))
#define max3(a,b,c) (a>b?(a>c?a:c):(b>c?b:c))
//#define min3(a,b,c) min(min(a,b),c)
//#define max3(a,b,c) max(max(a,b),c)

aabb_t*
  aabb_new_from_triangle(triangle_t* triangle);

aabb_t*
  aabb_new_from_sphere(sphere_t* sphere);

aabb_t*
  aabb_new_from_primitive(primitive_t *prm);

void
  aabb_fit_triangle(aabb_t *bb, triangle_t* triangle);

void
  aabb_fit_sphere(aabb_t *bb, sphere_t *sphere);

void
  aabb_fit_primitive(aabb_t *bb, primitive_t* primitive);

void
  aabb_combine(aabb_t *dest, aabb_t *bb1, aabb_t *bb2);

void
  aabb_update_centroid(aabb_t *bb);

void
  aabb_destroy(aabb_t **bb);

int
  aabb_intersect(aabb_t *bb, ray_t *ray);
