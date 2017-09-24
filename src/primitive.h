#pragma once

#define SPHERE      1
#define TRIANGLE    2

#include "includes.h"

typedef struct
{

	int type;
	void* obj;
} primitive_t;

intersection_t 
  prm_intersect(primitive_t *prm, ray_t *ray);

void
  prm_destroy(primitive_t **prm);

primitive_t*
  prm_from_triangle(triangle_t *triangel);

primitive_t*
  prm_from_sphere(sphere_t *sphere);


