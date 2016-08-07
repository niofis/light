#pragma once

#include "camera.h"
#include "sphere.h"
#include "point_light.h"
#include "triangle_list.h"

typedef struct
{
	camera_t *camera;
	triangle_t *triangles;
	sphere_t *spheres;
	point_light_t *point_lights;
	material_t *materials;
	int num_triangles;
	int num_spheres;
	int num_point_lights;
	int num_materials;
} world_t;

world_t*
  world_new();

world_t *
  world_cornell();

void
  world_destroy(world_t **world);
