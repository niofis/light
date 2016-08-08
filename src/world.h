#pragma once

#include "camera.h"
#include "sphere.h"
#include "point_light.h"
#include "list.h"

typedef struct
{
	camera_t *camera;
	list_t *triangles;
	list_t *lights;
	list_t *materials;
} world_t;

world_t*
  world_new();

world_t *
  world_cornell();

void
  world_destroy(world_t **world);
