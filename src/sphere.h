#pragma once
#include "includes.h"

typedef struct
{
	v3_t center;
	material_t material;
	float radius;
} sphere_t;

sphere_t*
  sphere_new();

void
  sphere_destroy(sphere_t **sphere);

intersection_t
  sphere_intersects(sphere_t *sphere, ray_t *ray);
