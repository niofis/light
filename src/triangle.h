#pragma once

#include "vector3.h"
#include "material.h"
#include "ray.h"
#include "intersection.h"

typedef struct
{
	v3_t v0;
	v3_t v1;
	v3_t v2;
	v3_t edge1;
	v3_t edge2;
	v3_t normal;
	material_t *material;
} triangle_t;

triangle_t*
  triangle_new();

void
  triangle_destroy(triangle_t **triangle);

int
  triangle_intersects(const triangle_t *triangle, const ray_t *ray, intersection_t *result);

void
  triangle_update(triangle_t *tr);

void
  triangle_scale_uni(triangle_t *tr, float scale);
