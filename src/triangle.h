#pragma once

#include "vector3.h"
#include "material.h"
#include "ray.h"
#include "intersection.h"

typedef struct
{
	v3_t pt1;
	v3_t pt2;
	v3_t pt3;
	v3_t v1;
	v3_t edge1;
	v3_t edge2;
	v3_t normal;
	material_t *material;
} triangle_t;

triangle_t*
  triangle_new(int count);

void
  triangle_destroy(triangle_t **triangle);

int
  triangle_intersects(const triangle_t *triangle, const ray_t *ray, intersection_t *result);

void
  triangle_update(triangle_t *tr);
